import { afterAll, beforeAll, describe, expect, it } from 'vitest'
import { DETERMINISTIC_DID_PK, testMnemonic, wasm, wasmBuilt } from './support/wasm'

// Binding smoke only. The scan/cache/login behaviour needs a browser (window +
// fetch), so it is covered by the Rust wasm-pack headless tests and the native
// mock-client tests in `did_lookup::scan`; Node just checks the surface is wired.
const { deriveWalletKeys, lookupDid, loginWithSeed, rememberDid } = wasm ?? {}

describe.skipIf(!wasmBuilt)('pegin-wasm local DID lookup surface', () => {
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

  it('exposes the async lookup/login surface and the cache hook', () => {
    // Not invoked here: a cache miss would hit the live network. Behaviour is
    // covered by the Rust wasm-pack (browser) and native mock-client tests.
    expect(typeof lookupDid).toBe('function')
    expect(typeof loginWithSeed).toBe('function')
    expect(typeof rememberDid).toBe('function')
  })
})
