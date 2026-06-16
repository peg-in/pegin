import { afterAll, beforeAll, describe, expect, it } from 'vitest'
import { DETERMINISTIC_DID_PK, testMnemonic, wasm, wasmBuilt } from './support/wasm'

// Binding smoke only. The login flow needs the relay (resolve + session), so it is covered
// by the SDK orchestration test and the Rust wasm-pack headless tests; Node just checks the
// new no-chain-I/O surface (identityKey + signLogin) is wired.
const { deriveWalletKeys, identityKey, signLogin } = wasm ?? {}

describe.skipIf(!wasmBuilt)('pegin-wasm identity + login-signing surface', () => {
  let keys: ReturnType<typeof deriveWalletKeys>

  beforeAll(() => {
    keys = deriveWalletKeys(testMnemonic())
  })

  afterAll(() => {
    keys?.free()
  })

  it('derives a deterministic DID public key', () => {
    expect(keys.didPkHex).toBe(DETERMINISTIC_DID_PK)
  })

  it('exposes a 48-byte watch-only account key the relay resolves', () => {
    expect(keys.accountPkHex).toHaveLength(96)
    expect(identityKey(keys).accountPk).toBe(keys.accountPkHex)
  })

  it('signs a verifiable login for a relay-resolved DID', () => {
    const out = signLogin(keys, 'did:chia:1relayresolved', 0, 'https://rp.example', 3600, 'nonce-1')
    expect(out.jwt.split('.')).toHaveLength(3)
    expect(typeof out.challengeSig).toBe('string')
  })
})
