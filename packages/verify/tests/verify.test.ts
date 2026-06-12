import { afterAll, beforeAll, describe, expect, it } from 'vitest'
import { existsSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

import { verifyLogin, verifyToken } from '../src/index.js'

const TEST_AUD = 'https://demo.example'
const TEST_DID = 'did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu'

const wasmPath = join(
  dirname(fileURLToPath(import.meta.url)),
  '../../../crates/pegin-wasm/pkg-node/pegin_wasm.js',
)
const wasmBuilt = existsSync(wasmPath)

describe.skipIf(!wasmBuilt)('@pegin/verify login verification', () => {
  let deriveWalletKeys: (phrase: string) => {
    didPkHex: string
    didPublicKey: () => Uint8Array
    free: () => void
  }
  let mintJwt: (
    keys: ReturnType<typeof deriveWalletKeys>,
    did: string,
    aud: string,
    ttl: number,
    nonce?: string,
  ) => string
  let signChallenge: (keys: ReturnType<typeof deriveWalletKeys>, challenge: string) => string

  beforeAll(async () => {
    const mod = await import(wasmPath)
    ;({ deriveWalletKeys, mintJwt, signChallenge } = mod)
  })

  it('verifies ES256K JWT minted by pegin-wasm', async () => {
    const { deterministicTestPhrase } = await import(
      '../../../crates/pegin-wasm/test-support/deterministic-phrase.mjs'
    )
    const keys = deriveWalletKeys(deterministicTestPhrase())
    const token = mintJwt(keys, TEST_DID, TEST_AUD, 3600)
    const verified = verifyToken(token, TEST_AUD, undefined, Math.floor(Date.now() / 1000))
    expect(verified.did).toBe(TEST_DID)
    expect(verified.didPkHex).toBe(keys.didPkHex)
    keys.free()
  })

  it('verifies JWT plus BLS challenge signature', async () => {
    const { deterministicTestPhrase } = await import(
      '../../../crates/pegin-wasm/test-support/deterministic-phrase.mjs'
    )
    const keys = deriveWalletKeys(deterministicTestPhrase())
    const nonce = 'server-nonce-42'
    const token = mintJwt(keys, TEST_DID, TEST_AUD, 3600, nonce)
    const challengeSig = signChallenge(keys, nonce)
    const verified = await verifyLogin({
      jwt: token,
      expectedAud: TEST_AUD,
      challengeNonce: nonce,
      challengeSigHex: challengeSig,
    })
    expect(verified.did).toBe(TEST_DID)
    keys.free()
  })

  it('rejects wrong audience', async () => {
    const { deterministicTestPhrase } = await import(
      '../../../crates/pegin-wasm/test-support/deterministic-phrase.mjs'
    )
    const keys = deriveWalletKeys(deterministicTestPhrase())
    const token = mintJwt(keys, TEST_DID, TEST_AUD, 3600)
    expect(() => verifyToken(token, 'https://evil.example', undefined, Math.floor(Date.now() / 1000)))
      .toThrow(/audience mismatch/)
    keys.free()
  })
})
