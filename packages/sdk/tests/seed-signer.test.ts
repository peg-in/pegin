import { describe, expect, it } from 'vitest'
import {
  SeedSigner,
  type SeedSignerWasm,
  type WasmWalletKeys,
} from '../src/features/login/signers/seed-signer.js'

interface Call {
  op: string
  args: unknown[]
}

function stubWasm(calls: Call[]): SeedSignerWasm {
  const keys: WasmWalletKeys = {
    accountPkHex: 'acct-hex',
    free: () => calls.push({ op: 'free', args: [] }),
  }
  return {
    deriveWalletKeys: (mnemonic) => {
      calls.push({ op: 'derive', args: [mnemonic] })
      return keys
    },
    identityKey: (k) => ({ accountPk: k.accountPkHex }),
    signLogin: (_k, did, ownerIndex, aud, ttlSeconds, nonce) => {
      calls.push({ op: 'signLogin', args: [did, ownerIndex, aud, ttlSeconds, nonce] })
      return { jwt: 'a.b.c', ...(nonce ? { challengeSig: 'sig' } : {}) }
    },
  }
}

describe('SeedSigner (test-only)', () => {
  it('derives keys once and shares them across identityKey + signLogin', async () => {
    const calls: Call[] = []
    const signer = new SeedSigner('phrase', { loadWasm: async () => stubWasm(calls) })

    expect((await signer.identityKey()).accountPk).toBe('acct-hex')
    const signed = await signer.signLogin({
      did: 'did:chia:1x',
      ownerIndex: 3,
      aud: 'https://rp',
      nonce: 'n',
    })

    expect(signed.jwt).toBe('a.b.c')
    expect(signed.challengeSig).toBe('sig')
    expect(calls.filter((c) => c.op === 'derive')).toHaveLength(1)
    expect(calls.find((c) => c.op === 'signLogin')?.args).toEqual([
      'did:chia:1x',
      3,
      'https://rp',
      3600,
      'n',
    ])
  })

  it('passes a custom jwtTtlSeconds through to WASM', async () => {
    const calls: Call[] = []
    const signer = new SeedSigner('phrase', {
      jwtTtlSeconds: 900,
      loadWasm: async () => stubWasm(calls),
    })
    await signer.signLogin({ did: 'd', ownerIndex: 0, aud: 'a', nonce: 'n' })
    expect(calls.find((c) => c.op === 'signLogin')?.args[3]).toBe(900)
  })

  it('frees the key handle on dispose', async () => {
    const calls: Call[] = []
    const signer = new SeedSigner('phrase', { loadWasm: async () => stubWasm(calls) })
    await signer.identityKey()
    await signer.dispose()
    expect(calls.some((c) => c.op === 'free')).toBe(true)
  })
})
