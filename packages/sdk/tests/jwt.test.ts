import { afterAll, beforeAll, describe, expect, it } from 'vitest'
import { decodeJwtPayload } from '../src/shared/lib/jwt'
import { alternateTestPhrase, testMnemonic, wasm, wasmBuilt } from './support/wasm'

const { deriveWalletKeys, mintJwt, verifyJwt } = wasm ?? {}

const TEST_DID = 'did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb'
const TEST_AUD = 'https://demo.example'

describe.skipIf(!wasmBuilt)('pegin-wasm JWT mint + verify', () => {
  // Key derivation (BIP39 + BLS) dominates test time — derive once per suite.
  let keys: ReturnType<typeof deriveWalletKeys>
  let token: string

  beforeAll(() => {
    keys = deriveWalletKeys(testMnemonic())
    token = mintJwt(keys, TEST_DID, TEST_AUD, 3600)
  })

  afterAll(() => {
    keys.free()
  })

  it('mints a three-part ES256K JWT', () => {
    expect(token.split('.')).toHaveLength(3)
    expect(verifyJwt(token, TEST_AUD, undefined)).toBe(true)
  })

  it('payload contains iss, sub, aud, iat, exp, and cnf.did_pk', () => {
    const before = Math.floor(Date.now() / 1000)
    const fresh = mintJwt(keys, TEST_DID, TEST_AUD, 600, 'nonce-1')
    const after = Math.floor(Date.now() / 1000)
    const payload = decodeJwtPayload(fresh)

    expect(payload.iss).toBe(TEST_DID)
    expect(payload.sub).toBe(TEST_DID)
    expect(payload.aud).toBe(TEST_AUD)
    expect(payload.nonce).toBe('nonce-1')

    const cnf = payload.cnf as { did_pk?: string }
    expect(cnf.did_pk).toBe(keys.didPkHex)

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

    expect(verifyJwt(`${header}.${evil}.${sig}`, TEST_AUD, undefined)).toBe(false)
  })

  it('rejects verification with a different audience', () => {
    expect(verifyJwt(token, 'https://other.example', undefined)).toBe(false)
  })

  it('returns false for malformed tokens', () => {
    expect(verifyJwt('not-a-jwt', TEST_AUD, undefined)).toBe(false)
  })

  // A self-signed token verifies against the key embedded in its own `cnf.did_pk`,
  // so any wallet's token is internally valid. Binding that key to a real identity is
  // the relying party's job (challenge signature + on-chain DID owner check), not verifyJwt's.
  it('accepts a self-signed token from any key (cnf.did_pk is self-asserted)', () => {
    const otherKeys = deriveWalletKeys(alternateTestPhrase())
    const otherToken = mintJwt(otherKeys, TEST_DID, TEST_AUD, 3600)
    expect(verifyJwt(otherToken, TEST_AUD, undefined)).toBe(true)
    otherKeys.free()
  })

  it('rejects a token whose signature was swapped for another key’s', () => {
    const otherKeys = deriveWalletKeys(alternateTestPhrase())
    const otherToken = mintJwt(otherKeys, TEST_DID, TEST_AUD, 3600)
    // Splice this token's cnf.did_pk header+payload onto the other token's signature.
    const [header, payload] = token.split('.')
    const otherSig = otherToken.split('.')[2]
    expect(verifyJwt(`${header}.${payload}.${otherSig}`, TEST_AUD, undefined)).toBe(false)
    otherKeys.free()
  })
})
