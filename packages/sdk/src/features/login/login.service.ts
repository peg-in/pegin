import type { PeginSession } from '../../entities/session/index.js'
import { PeginAuthClient } from '../../shared/api/pegin-auth-api.js'
import type { PeginSigner } from './signers/pegin-signer.js'
import { PasskeySigner, type PasskeySignerOptions } from './signers/passkey-signer.js'

export interface LoginWithPeginOptions {
  /** How the login is signed (PasskeySigner in production; SeedSigner in tests). */
  signer: PeginSigner
  /** Same-origin auth API prefix. Default `/api/pegin`. */
  apiPrefix?: string
}

export interface LoginWithPasskeyOptions extends PasskeySignerOptions {
  /** Same-origin auth API prefix. Default `/api/pegin`. */
  apiPrefix?: string
}

const DEFAULT_API_PREFIX = '/api/pegin'

/**
 * Full secure login: account key → relay resolve → signer proves possession → session.
 *
 * The browser never reads the chain: the relay maps the signer's watch-only `accountPk` to
 * `{ did, ownerIndex }`, the signer mints the JWT for that owner, and the relay re-verifies
 * on-chain ownership (feat-17). `aud` comes from the server (request Origin), never hardcoded.
 */
export async function loginWithPegin(options: LoginWithPeginOptions): Promise<PeginSession> {
  const { signer } = options
  const auth = new PeginAuthClient(options.apiPrefix ?? DEFAULT_API_PREFIX)

  // The account key gates resolution; nonce issuance is independent, so overlap them.
  const { accountPk } = await signer.identityKey()
  const [{ loginId, nonce, aud }, resolved] = await Promise.all([
    auth.requestNonce(),
    auth.resolve(accountPk),
  ])

  const { jwt, challengeSig } = await signer.signLogin({
    did: resolved.did,
    ownerIndex: resolved.ownerIndex,
    aud,
    nonce,
  })
  const session = await auth.completeLogin({
    loginId,
    jwt,
    ...(challengeSig !== undefined ? { challengeSig } : {}),
  })
  return toPeginSession(session, jwt)
}

/**
 * Passkey login in one call: build a `PasskeySigner`, run the secure login, and dispose the
 * key handle whether it succeeds or throws. The default production path — no seed is typed,
 * one biometric prompt unlocks the wallet, and a verified server session comes back.
 */
export async function loginWithPasskey(options: LoginWithPasskeyOptions): Promise<PeginSession> {
  const { apiPrefix, ...signerOptions } = options
  const signer = new PasskeySigner({
    ...signerOptions,
    ...(apiPrefix !== undefined ? { authApiPrefix: apiPrefix } : {}),
  })
  try {
    return await loginWithPegin({
      signer,
      ...(apiPrefix !== undefined ? { apiPrefix } : {}),
    })
  } finally {
    // Zeroize the in-memory wallet keys regardless of outcome.
    await signer.dispose().catch(() => undefined)
  }
}

/** Restores a server-verified session, or null when logged out / expired. */
export async function loadPeginSession(
  apiPrefix = DEFAULT_API_PREFIX,
): Promise<PeginSession | null> {
  const session = await new PeginAuthClient(apiPrefix).getSession()
  return session ? toPeginSession(session) : null
}

/** Clears the server HttpOnly session cookie. */
export async function logoutPegin(apiPrefix = DEFAULT_API_PREFIX): Promise<void> {
  await new PeginAuthClient(apiPrefix).logout()
}

// `jwt` is only available on fresh login (minted client-side); session restore
// reads it from the HttpOnly cookie server-side, so it defaults to empty there.
function toPeginSession(
  session: { did: string; sub: string; expiresAt: number },
  jwt = '',
): PeginSession {
  return {
    did: session.did,
    sub: session.sub,
    expiresAt: session.expiresAt,
    username: session.did,
    jwt,
  }
}
