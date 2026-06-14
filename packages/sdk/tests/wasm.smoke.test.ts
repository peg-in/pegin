import { describe, expect, it } from 'vitest'
import {
  alternateTestPhrase,
  DETERMINISTIC_DID_PK,
  DETERMINISTIC_WALLET_PK,
  testMnemonic,
  wasm,
  wasmBuilt,
} from './support/wasm'

const { deriveWalletKeys, deriveKeys } = wasm ?? {}

describe.skipIf(!wasmBuilt)('pegin-wasm smoke test', () => {
  it('derives keys matching known BLS vectors', () => {
    const keys = deriveWalletKeys(testMnemonic())
    expect(keys.walletPkHex).toBe(DETERMINISTIC_WALLET_PK)
    expect(keys.didPkHex).toBe(DETERMINISTIC_DID_PK)
    expect(Array.from(keys.didPublicKey)).toHaveLength(48)
    keys.free()
  })

  it('deriveKeys alias matches deriveWalletKeys', () => {
    const phrase = testMnemonic()
    const a = deriveWalletKeys(phrase)
    const b = deriveKeys(phrase)
    expect(a.didPkHex).toBe(b.didPkHex)
    expect(a.walletPkHex).toBe(b.walletPkHex)
    a.free()
    b.free()
  })

  it('is deterministic for the same mnemonic', () => {
    const phrase = testMnemonic()
    const a = deriveWalletKeys(phrase)
    const b = deriveWalletKeys(phrase)
    expect(a.didPkHex).toBe(b.didPkHex)
    expect(a.walletPkHex).toBe(b.walletPkHex)
    a.free()
    b.free()
  })

  it('wallet and DID keys differ', () => {
    const keys = deriveWalletKeys(testMnemonic())
    expect(keys.walletPkHex).not.toBe(keys.didPkHex)
    keys.free()
  })

  it('rejects an invalid mnemonic', () => {
    expect(() => deriveWalletKeys('not a valid mnemonic')).toThrow('invalid mnemonic')
  })
})
