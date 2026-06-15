import type { PeginSession } from '../../entities/session/index.js'
import { PeginAuthClient } from '../../shared/api/pegin-auth-api.js'
import { logger } from '../../shared/lib/logger.js'

/** Identity material minted in WASM — secrets never leave the browser. */
export interface PeginWalletLogin {
  did: string
  jwt: string
  challengeSig?: string
  walletFp: string
  ownerIndex: number
}

/**
 * Minimal WASM surface required for seed login. Resolution reads only public chain
 * data (coinset hints) and caches the result; secrets never cross to the relay.
 */
export interface PeginWasmLogin {
  loginWithSeed(
    mnemonic: string,
    scan_limit: number,
    ttl_seconds: number,
    aud: string,
    challenge_nonce?: string | null,
  ): Promise<PeginWalletLogin>
  /** Caches the relay-confirmed DID so the next login skips the on-chain scan. */
  rememberDid?(walletFp: string, did: string, ownerIndex: number): void
}

export interface LoginWithPeginOptions {
  /** Same-origin auth API prefix. Default `/api/pegin`. */
  apiPrefix?: string
  /** JWT lifetime passed to WASM. Default 3600. */
  jwtTtlSeconds?: number
  /** Highest address index the first-login DID scan probes. Default 0 → WASM default (10 000). */
  scanLimit?: number
  /** Loads the browser WASM module (must alias `@pegin/wasm` in bundler). */
  loadWasm?: () => Promise<PeginWasmLogin>
}

const DEFAULT_API_PREFIX = '/api/pegin'

/**
 * Full secure login: server nonce → WASM mint → server verify → HttpOnly session.
 * `aud` comes from the server (derived from request Origin), never hardcoded.
 */
export async function loginWithPegin(
  mnemonic: string,
  options: LoginWithPeginOptions = {},
): Promise<PeginSession> {
  const auth = new PeginAuthClient(options.apiPrefix ?? DEFAULT_API_PREFIX)
  // Nonce fetch and WASM init are independent; overlap them to save one round-trip.
  const [{ loginId, nonce, aud }, wasm] = await Promise.all([
    auth.requestNonce(),
    (options.loadWasm ?? defaultLoadWasm)(),
  ])
  const wallet = await wasm.loginWithSeed(
    mnemonic,
    options.scanLimit ?? 0,
    options.jwtTtlSeconds ?? 3600,
    aud,
    nonce,
  )
  const session = await auth.completeLogin({
    loginId,
    jwt: wallet.jwt,
    ...(wallet.challengeSig !== undefined ? { challengeSig: wallet.challengeSig } : {}),
  })
  // Cache the relay-confirmed DID so the next login skips the on-chain scan. Best-effort:
  // the user is already authenticated, so a cache failure must not reject the login.
  try {
    wasm.rememberDid?.(wallet.walletFp, session.did, wallet.ownerIndex)
  } catch (err) {
    logger.warn('rememberDid cache write failed; continuing with the session', err)
  }
  return toPeginSession(session, wallet.jwt)
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

async function defaultLoadWasm(): Promise<PeginWasmLogin> {
  const mod = await import('@pegin/wasm')
  await mod.default()
  return {
    loginWithSeed: mod.loginWithSeed.bind(mod),
    rememberDid: mod.rememberDid.bind(mod),
  }
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
