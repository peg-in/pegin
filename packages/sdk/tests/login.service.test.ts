import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { loginWithPasskey, loginWithPegin } from '../src/features/login/login.service.js'
import { enrollPasskey, type PasskeyWasm } from '../src/features/login/signers/passkey-signer.js'
import type {
  PasskeyVaultStore,
  VaultBlob,
} from '../src/features/login/signers/passkey-vault.js'
import type { PeginSigner, SignLoginRequest } from '../src/features/login/signers/pegin-signer.js'

const TEST_DID = 'did:chia:1relayresolved'
const TEST_AUD = 'https://rp.example'

// Orchestration test: the relay is mocked and the signer is stubbed, so it asserts the SDK
// wiring (identity → nonce + resolve → sign → session) without a real chain read.
describe('loginWithPegin', () => {
  const signLoginCalls: SignLoginRequest[] = []

  const signer: PeginSigner = {
    identityKey: async () => ({ accountPk: 'acct-pk-hex' }),
    signLogin: async (req) => {
      signLoginCalls.push(req)
      return { jwt: 'header.payload.sig', challengeSig: 'ab12' }
    },
  }

  beforeEach(() => {
    signLoginCalls.length = 0
    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, init?: RequestInit) => {
        if (url.endsWith('/nonce') && init?.method === 'POST') {
          return json({ loginId: 'login-1', nonce: 'relay-nonce', aud: TEST_AUD })
        }
        if (url.endsWith('/resolve') && init?.method === 'POST') {
          return json({ did: TEST_DID, ownerIndex: 4757 })
        }
        if (url.endsWith('/session') && init?.method === 'POST') {
          return json({ did: TEST_DID, sub: TEST_DID, expiresAt: 9_999_999_999 })
        }
        return new Response('not found', { status: 404 })
      }),
    )
  })

  afterEach(() => {
    vi.unstubAllGlobals()
  })

  it('orchestrates identity → nonce + resolve → sign → session', async () => {
    const session = await loginWithPegin({ signer, apiPrefix: '/api/pegin' })

    expect(session.did).toBe(TEST_DID)
    expect(session.jwt).toBe('header.payload.sig')

    // The signer signs for the relay-resolved DID + owner index with the server nonce/aud.
    expect(signLoginCalls).toEqual([
      { did: TEST_DID, ownerIndex: 4757, aud: TEST_AUD, nonce: 'relay-nonce' },
    ])

    const paths = vi.mocked(fetch).mock.calls.map((c) => String(c[0]))
    expect(paths).toContain('/api/pegin/nonce')
    expect(paths).toContain('/api/pegin/resolve')
    expect(paths).toContain('/api/pegin/session')
  })

  it('omits challengeSig from the session call when the signer returns none', async () => {
    const noChallenge: PeginSigner = {
      identityKey: signer.identityKey,
      signLogin: async () => ({ jwt: 'a.b.c' }),
    }
    await loginWithPegin({ signer: noChallenge })

    const sessionCall = vi
      .mocked(fetch)
      .mock.calls.find((c) => String(c[0]).endsWith('/session'))
    const body = JSON.parse((sessionCall?.[1] as RequestInit).body as string) as Record<
      string,
      unknown
    >
    expect(body).not.toHaveProperty('challengeSig')
  })
})

// loginWithPasskey is the production one-call path: enroll once, then it builds a
// PasskeySigner, runs the secure login, and disposes the key handle for us.
describe('loginWithPasskey', () => {
  beforeEach(() => {
    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, init?: RequestInit) => {
        if (url.endsWith('/nonce') && init?.method === 'POST') {
          return json({ loginId: 'login-1', nonce: 'relay-nonce', aud: TEST_AUD })
        }
        if (url.endsWith('/resolve') && init?.method === 'POST') {
          return json({ did: TEST_DID, ownerIndex: 4757 })
        }
        if (url.endsWith('/session') && init?.method === 'POST') {
          return json({ did: TEST_DID, sub: TEST_DID, expiresAt: 9_999_999_999 })
        }
        return new Response('not found', { status: 404 })
      }),
    )
  })

  afterEach(() => {
    vi.unstubAllGlobals()
  })

  it('enrolls once, then logs in with the passkey and disposes the key handle', async () => {
    const vault = memoryVault()
    const freed = { count: 0 }
    const loadWasm = async (): Promise<PasskeyWasm> => passkeyWasm(freed)
    const webauthn = prfWebauthn()

    await enrollPasskey({ rpId: 'app.example', userName: 'alice', mnemonic: 'seed', webauthn, vault, loadWasm })

    const session = await loginWithPasskey({ rpId: 'app.example', webauthn, vault, loadWasm })

    expect(session.did).toBe(TEST_DID)
    expect(session.jwt).toBe(`${TEST_DID}|4757|${TEST_AUD}|3600`)
    // dispose() ran in the finally block, zeroizing the in-memory keys.
    expect(freed.count).toBe(1)
  })
})

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

function prfWebauthn() {
  const secret = new Uint8Array(32).fill(7)
  const credential = (prf: unknown) =>
    ({ rawId: new ArrayBuffer(16), getClientExtensionResults: () => ({ prf }) }) as unknown as Credential
  return {
    create: async () => credential({ enabled: true }),
    get: async () => credential({ results: { first: secret.buffer } }),
  }
}

function passkeyWasm(freed: { count: number }): PasskeyWasm {
  const keys = {
    accountPkHex: 'acct-hex',
    free: () => {
      freed.count += 1
    },
  }
  return {
    deriveWalletKeys: () => keys,
    identityKey: (k) => ({ accountPk: k.accountPkHex }),
    signLogin: (_k, did, ownerIndex, aud, ttl, nonce) => ({
      jwt: `${did}|${ownerIndex}|${aud}|${ttl}`,
      ...(nonce ? { challengeSig: 'sig' } : {}),
    }),
  }
}

function json(payload: unknown): Response {
  return new Response(JSON.stringify(payload), {
    status: 200,
    headers: { 'content-type': 'application/json' },
  })
}
