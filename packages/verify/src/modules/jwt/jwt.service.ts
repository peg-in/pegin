import { secp256k1 } from '@noble/curves/secp256k1'
import { sha256 } from '@noble/hashes/sha2'

import { decodeBase64Url } from '../../shared/lib/base64url.js'
import { VerifyError } from '../../shared/types/verify.types.js'

interface JwtHeader {
  alg?: string
  jwk?: { kty?: string; crv?: string; x?: string; y?: string }
}

interface JwtPayload {
  iss?: string
  sub?: string
  aud?: string
  iat?: number
  exp?: number
  nonce?: string
  cnf?: { did_pk?: string }
}

export interface VerifiedJwt {
  did: string
  aud: string
  didPkHex: string
  nonce?: string
}

function splitToken(token: string): [string, string, string] {
  const parts = token.split('.')
  if (parts.length !== 3) {
    throw new VerifyError('JWT must have exactly 3 dot-separated parts')
  }
  const [header, payload, sig] = parts
  if (!header || !payload || !sig) {
    throw new VerifyError('JWT must have exactly 3 dot-separated parts')
  }
  return [header, payload, sig]
}

function decodeJson(segment: string, name: string): unknown {
  try {
    const text = new TextDecoder().decode(decodeBase64Url(segment))
    return JSON.parse(text)
  } catch {
    throw new VerifyError(`invalid ${name}`)
  }
}

function decodeCoord(value: string | undefined, name: string): Uint8Array {
  if (!value) throw new VerifyError(`${name} missing`)
  const bytes = decodeBase64Url(value)
  if (bytes.length !== 32) throw new VerifyError(`${name} must be 32 bytes`)
  return bytes
}

function verifyEs256kSignature(header: JwtHeader, signingInput: string, sigSegment: string): void {
  const jwk = header.jwk
  if (!jwk) throw new VerifyError('ES256K JWT header missing jwk')
  const x = decodeCoord(jwk.x, 'jwk.x')
  const y = decodeCoord(jwk.y, 'jwk.y')
  const publicKey = new Uint8Array(65)
  publicKey[0] = 0x04
  publicKey.set(x, 1)
  publicKey.set(y, 33)
  const sigBytes = decodeBase64Url(sigSegment)
  if (sigBytes.length !== 64) throw new VerifyError('invalid ES256K signature length')
  const digest = sha256(signingInput)
  const ok = secp256k1.verify(sigBytes, digest, publicKey, { prehash: true })
  if (!ok) throw new VerifyError('invalid JWT signature')
}

function validateClaims(
  payload: JwtPayload,
  expectedAud: string,
  expectedNonce: string | undefined,
  now: number,
): VerifiedJwt {
  const exp = payload.exp
  const iss = payload.iss
  const sub = payload.sub
  const aud = payload.aud
  if (exp === undefined || exp < now) throw new VerifyError('JWT expired')
  if (!iss || !sub || iss !== sub) throw new VerifyError('iss and sub must match')
  if (!aud || aud !== expectedAud) {
    throw new VerifyError(`audience mismatch: expected ${expectedAud}, got ${aud ?? ''}`)
  }
  if (expectedNonce !== undefined) {
    if (payload.nonce !== expectedNonce) throw new VerifyError('nonce mismatch')
  }
  const didPkHex = payload.cnf?.did_pk
  if (!didPkHex) throw new VerifyError('cnf.did_pk missing from JWT')
  return {
    did: iss,
    aud,
    didPkHex,
    ...(payload.nonce !== undefined ? { nonce: payload.nonce } : {}),
  }
}

/** Verifies a PEGIN ES256K JWT and returns extracted claims. */
export function verifyToken(
  token: string,
  expectedAud: string,
  expectedNonce: string | undefined,
  now: number,
): VerifiedJwt {
  const [headerB64, payloadB64, sigB64] = splitToken(token)
  const header = decodeJson(headerB64, 'JWT header') as JwtHeader
  if (header.alg !== 'ES256K') {
    throw new VerifyError(`unsupported JWT algorithm: ${header.alg ?? 'missing'}`)
  }
  const signingInput = `${headerB64}.${payloadB64}`
  verifyEs256kSignature(header, signingInput, sigB64)
  const payload = decodeJson(payloadB64, 'JWT payload') as JwtPayload
  return validateClaims(payload, expectedAud, expectedNonce, now)
}
