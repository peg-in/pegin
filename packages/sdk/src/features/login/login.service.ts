import type { PeginSession } from '../../entities/session/index.js'
import { PeginAuthClient } from '../../shared/api/pegin-auth-api.js'

/** Minimal WASM surface required for seed login. */
export interface PeginWasmLogin {
  loginWithSeed(
    mnemonic: string,
    peer_url: string | null | undefined,
    ttl_seconds: number,
    aud: string,
    challenge_nonce?: string | null,
  ): Promise<{ did: string; jwt: string; challengeSig?: string }>
}

export interface LoginWithPeginOptions {
  /** Same-origin auth API prefix. Default `/api/pegin`. */
  apiPrefix?: string
  /** JWT lifetime passed to WASM. Default 3600. */
  jwtTtlSeconds?: number
  /** Coinset peer for on-chain DID lookup. Default null (testnet11). */
  peerUrl?: string | null
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
    options.peerUrl ?? null,
    options.jwtTtlSeconds ?? 3600,
    aud,
    nonce,
  )
  const session = await auth.completeLogin({
    loginId,
    jwt: wallet.jwt,
    ...(wallet.challengeSig !== undefined ? { challengeSig: wallet.challengeSig } : {}),
  })
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
  return { loginWithSeed: mod.loginWithSeed.bind(mod) }
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
