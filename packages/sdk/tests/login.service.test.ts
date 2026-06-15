import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import {
  loginWithPegin,
  type PeginWalletLogin,
  type PeginWasmLogin,
} from '../src/features/login/login.service.js'

const TEST_DID = 'did:chia:1relayresolved'
const TEST_AUD = 'https://rp.example'

// Orchestration test: the relay is mocked and WASM is stubbed, so it asserts the
// SDK wiring (nonce → local login → session → cache) without a real chain scan.
describe('loginWithPegin', () => {
  const seedCalls: Array<{ scanLimit: number; aud: string; nonce?: string | null }> = []
  const rememberCalls: Array<{ walletFp: string; did: string; ownerIndex: number }> = []

  const wallet: PeginWalletLogin = {
    did: '',
    jwt: 'header.payload.sig',
    challengeSig: 'ab12',
    walletFp: 'fp-deadbeef',
    ownerIndex: 4757,
  }

  const stubWasm: PeginWasmLogin = {
    loginWithSeed: async (_mnemonic, scanLimit, _ttl, aud, nonce) => {
      seedCalls.push({ scanLimit, aud, nonce })
      return wallet
    },
    rememberDid: (walletFp, did, ownerIndex) => {
      rememberCalls.push({ walletFp, did, ownerIndex })
    },
  }

  beforeEach(() => {
    seedCalls.length = 0
    rememberCalls.length = 0
    vi.stubGlobal(
      'fetch',
      vi.fn(async (url: string, init?: RequestInit) => {
        if (url.endsWith('/nonce') && init?.method === 'POST') {
          return new Response(
            JSON.stringify({ loginId: 'login-1', nonce: 'relay-nonce', aud: TEST_AUD }),
            { status: 200, headers: { 'content-type': 'application/json' } },
          )
        }
        if (url.endsWith('/session') && init?.method === 'POST') {
          return new Response(
            JSON.stringify({ did: TEST_DID, sub: TEST_DID, expiresAt: 9_999_999_999 }),
            { status: 200, headers: { 'content-type': 'application/json' } },
          )
        }
        return new Response('not found', { status: 404 })
      }),
    )
  })

  afterEach(() => {
    vi.unstubAllGlobals()
  })

  it('orchestrates nonce → local WASM login → relay session only', async () => {
    const session = await loginWithPegin('test mnemonic', {
      apiPrefix: '/api/pegin',
      scanLimit: 5000,
      loadWasm: async () => stubWasm,
    })

    expect(session.did).toBe(TEST_DID)
    expect(session.jwt).toBe(wallet.jwt)

    // The relay receives the server nonce and scan limit; nothing else.
    expect(seedCalls).toEqual([{ scanLimit: 5000, aud: TEST_AUD, nonce: 'relay-nonce' }])

    const fetchMock = vi.mocked(fetch)
    expect(fetchMock).toHaveBeenCalledTimes(2)
    expect(fetchMock.mock.calls[0]?.[0]).toBe('/api/pegin/nonce')
    expect(fetchMock.mock.calls[1]?.[0]).toBe('/api/pegin/session')

    // The relay-confirmed DID is cached against the owning index for next time.
    expect(rememberCalls).toEqual([{ walletFp: wallet.walletFp, did: TEST_DID, ownerIndex: 4757 }])
  })

  it('defaults scanLimit to 0 (WASM default ceiling) when unset', async () => {
    await loginWithPegin('test mnemonic', { loadWasm: async () => stubWasm })
    expect(seedCalls[0]?.scanLimit).toBe(0)
  })
})
