import { afterAll, beforeAll, describe, expect, it } from 'vitest'
import { decodeJwtPayload } from '../src/shared/lib/jwt'
import { alternateTestPhrase, testMnemonic, wasm, wasmBuilt } from './support/wasm'

const { deriveWalletKeys, mintJwt, verifyJwt } = wasm ?? {}

const TEST_DID = 'did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb'

describe.skipIf(!wasmBuilt)('pegin-wasm JWT mint + verify', () => {
  // Key derivation (BIP39 + BLS) dominates test time — derive once per suite.
  let keys: ReturnType<typeof deriveWalletKeys>
  let otherKeys: ReturnType<typeof deriveWalletKeys>
  let token: string

  beforeAll(() => {
    keys = deriveWalletKeys(testMnemonic())
    otherKeys = deriveWalletKeys(alternateTestPhrase())
    token = mintJwt(keys, TEST_DID, 3600)
  })

  afterAll(() => {
    keys.free()
    otherKeys.free()
  })

  it('mints a three-part JWT signed with the DID key', () => {
    expect(token.split('.')).toHaveLength(3)
    expect(verifyJwt(token, keys.didPublicKey)).toBe(true)
  })

  it('payload contains iss, sub, iat, and exp only', () => {
    const before = Math.floor(Date.now() / 1000)
    const fresh = mintJwt(keys, TEST_DID, 600)
    const after = Math.floor(Date.now() / 1000)
    const payload = decodeJwtPayload(fresh)

    expect(payload.iss).toBe(TEST_DID)
    expect(payload.sub).toBe(TEST_DID)
    expect(payload.aud).toBeUndefined()

    const iat = payload.iat as number
    const exp = payload.exp as number
    expect(iat).toBeGreaterThanOrEqual(before)
    expect(iat).toBeLessThanOrEqual(after)
    expect(exp).toBe(iat + 600)
  })

  it('rejects a tampered payload', () => {
    const [header, , sig] = token.split('.')
    const evil = Buffer.from(
      JSON.stringify({ iss: 'attacker', sub: 'attacker', exp: 9_999_999_999 }),
    ).toString('base64url')

    expect(verifyJwt(`${header}.${evil}.${sig}`, keys.didPublicKey)).toBe(false)
  })

  it('rejects verification with a different public key', () => {
    expect(verifyJwt(token, otherKeys.didPublicKey)).toBe(false)
  })

  it('returns false for malformed tokens', () => {
    expect(verifyJwt('not-a-jwt', keys.didPublicKey)).toBe(false)
  })
})
