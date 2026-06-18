import { describe, expect, it } from 'vitest'
import {
  enrollPasskey,
  PasskeySigner,
  type PasskeyWasm,
  type WasmWalletKeys,
  type WebAuthnApi,
} from '../src/features/login/signers/passkey-signer.js'
import type { PasskeyVaultStore, VaultBlob } from '../src/features/login/signers/passkey-vault.js'

const PRF_SECRET = new Uint8Array(32).fill(7)
const SEED = 'unit test seed phrase that the passkey guards'

function credentialWith(prf: unknown, rawId = new ArrayBuffer(16)): Credential {
  return {
    rawId,
    getClientExtensionResults: () => ({ prf }),
  } as unknown as Credential
}

// A PRF-capable authenticator: create() makes a credential, get() yields the PRF secret.
function fakeWebauthn(secret = PRF_SECRET): WebAuthnApi {
  return {
    create: async () => credentialWith({ enabled: true }),
    get: async () => credentialWith({ results: { first: secret.buffer } }),
  }
}

function memoryVault(): PasskeyVaultStore {
  let blob: VaultBlob | null = null
  return {
    load: () => blob,
    save: (b) => {
      blob = b
    },
    clear: () => {
      blob = null
    },
  }
}

function stubWasm(record: { mnemonics: string[] }): PasskeyWasm {
  const keys: WasmWalletKeys = { accountPkHex: 'acct-hex', free: () => undefined }
  return {
    deriveWalletKeys: (mnemonic) => {
      record.mnemonics.push(mnemonic)
      return keys
    },
    identityKey: (k) => ({ accountPk: k.accountPkHex }),
    signLogin: (_k, did, ownerIndex, aud, ttl, nonce) => ({
      jwt: `${did}|${ownerIndex}|${aud}|${ttl}`,
      ...(nonce ? { challengeSig: 'sig' } : {}),
    }),
  }
}

describe('PasskeySigner (PRF-wraps-seed)', () => {
  it('enrolls a seed under the passkey, then logs in by decrypting it', async () => {
    const webauthn = fakeWebauthn()
    const vault = memoryVault()
    const record = { mnemonics: [] as string[] }

    await enrollPasskey({
      rpId: 'app.example',
      userName: 'alice',
      mnemonic: SEED,
      webauthn,
      vault,
      loadWasm: async () => stubWasm(record),
    })
    // The seed is stored encrypted — never in the clear.
    expect(vault.load()?.ct).toBeTruthy()
    expect(JSON.stringify(vault.load())).not.toContain('seed phrase')

    const signer = new PasskeySigner({
      rpId: 'app.example',
      webauthn,
      vault,
      loadWasm: async () => stubWasm(record),
    })
    expect((await signer.identityKey()).accountPk).toBe('acct-hex')
    const signed = await signer.signLogin({
      did: 'did:chia:1x',
      ownerIndex: 4,
      aud: 'https://rp',
      nonce: 'n',
    })

    expect(signed.jwt).toBe('did:chia:1x|4|https://rp|3600')
    expect(signed.challengeSig).toBe('sig')
    // The decrypted seed — not the PRF secret — drives BLS derivation.
    expect(record.mnemonics).toContain(SEED)
  })

  it('refuses to log in before enrollment', async () => {
    const signer = new PasskeySigner({
      rpId: 'app.example',
      webauthn: fakeWebauthn(),
      vault: memoryVault(),
      loadWasm: async () => stubWasm({ mnemonics: [] }),
    })
    await expect(signer.identityKey()).rejects.toThrow(/enroll first/)
  })

  it('fails to unlock when a different passkey yields the wrong PRF secret', async () => {
    const vault = memoryVault()
    await enrollPasskey({
      rpId: 'app.example',
      userName: 'alice',
      mnemonic: SEED,
      webauthn: fakeWebauthn(PRF_SECRET),
      vault,
      loadWasm: async () => stubWasm({ mnemonics: [] }),
    })

    const wrongSecret = new Uint8Array(32).fill(9)
    const signer = new PasskeySigner({
      rpId: 'app.example',
      webauthn: fakeWebauthn(wrongSecret),
      vault,
      loadWasm: async () => stubWasm({ mnemonics: [] }),
    })
    await expect(signer.identityKey()).rejects.toThrow()
  })

  it('throws when the authenticator returns no PRF secret', async () => {
    const vault = memoryVault()
    vault.save({ iv: 'AAAA', ct: 'AAAA' })
    const signer = new PasskeySigner({
      rpId: 'app.example',
      webauthn: { create: async () => null, get: async () => credentialWith({ enabled: true }) },
      vault,
      loadWasm: async () => stubWasm({ mnemonics: [] }),
    })
    await expect(signer.identityKey()).rejects.toThrow(/PRF secret/)
  })
})
