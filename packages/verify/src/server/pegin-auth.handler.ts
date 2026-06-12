import { randomBytes } from 'node:crypto'
import type { IncomingMessage, ServerResponse } from 'node:http'

import { verifyLogin } from '../index.js'
import { VerifyError } from '../shared/types/verify.types.js'
import type {
  PeginCompleteLoginBody,
  PeginNonceResponse,
  PeginSessionResponse,
} from './auth.dto.js'

interface PendingLogin {
  nonce: string
  aud: string
  expiresAt: number
}

interface StoredSession {
  did: string
  expiresAt: number
}

const SESSION_COOKIE = 'pegin_session'
/** Removed in favor of JSON `loginId`; cleared when present from older dev servers. */
const LEGACY_NONCE_COOKIE = 'pegin_login_nonce'

const pendingLogins = new Map<string, PendingLogin>()
const sessions = new Map<string, StoredSession>()

export interface PeginAuthHandlerOptions {
  coinsetBaseUrl?: string
  sessionTtlSeconds?: number
}

/** Node/Vite middleware — mount at `/api/pegin` (handles `/nonce`, `/session`, `/logout`). */
export function createPeginAuthMiddleware(options: PeginAuthHandlerOptions = {}) {
  const sessionTtl = options.sessionTtlSeconds ?? 3600

  return async (req: IncomingMessage, res: ServerResponse, next: () => void): Promise<void> => {
    const url = new URL(req.url ?? '/', 'http://localhost')
    const path = url.pathname.replace(/\/$/, '') || '/'

    try {
      if (req.method === 'POST' && path === '/nonce') {
        handleNonce(req, res)
        return
      }
      if (req.method === 'POST' && path === '/session') {
        await handleSession(req, res, options, sessionTtl)
        return
      }
      if (req.method === 'GET' && path === '/session') {
        handleGetSession(req, res)
        return
      }
      if (req.method === 'POST' && path === '/logout') {
        handleLogout(req, res)
        return
      }
    } catch (err) {
      sendJson(res, 500, { error: err instanceof Error ? err.message : 'internal error' })
      return
    }

    next()
  }
}

function handleNonce(req: IncomingMessage, res: ServerResponse): void {
  purgeExpired()
  const aud = audienceFromRequest(req)
  const nonce = randomBytes(32).toString('base64url')
  const loginId = randomBytes(16).toString('hex')
  pendingLogins.set(loginId, { nonce, aud, expiresAt: Date.now() + 5 * 60_000 })
  const body: PeginNonceResponse = { loginId, nonce, aud }
  sendJson(res, 200, body)
}

async function handleSession(
  req: IncomingMessage,
  res: ServerResponse,
  options: PeginAuthHandlerOptions,
  sessionTtl: number,
): Promise<void> {
  purgeExpired()
  const body = await readJsonBody<PeginCompleteLoginBody>(req)
  const pending = pendingLogins.get(body.loginId)
  pendingLogins.delete(body.loginId)
  if (!pending || pending.expiresAt < Date.now()) {
    sendJson(res, 401, { error: 'login expired — start again' })
    return
  }

  const requestAud = audienceFromRequest(req)
  if (requestAud !== pending.aud) {
    sendJson(res, 403, { error: 'audience mismatch' })
    return
  }

  let verified
  try {
    verified = await verifyLogin({
      jwt: body.jwt,
      expectedAud: pending.aud,
      challengeNonce: pending.nonce,
      ...(body.challengeSig !== undefined ? { challengeSigHex: body.challengeSig } : {}),
      ...(options.coinsetBaseUrl !== undefined ? { coinsetBaseUrl: options.coinsetBaseUrl } : {}),
    })
  } catch (err) {
    // A rejected login is the client's fault (bad/forged token) — 401, not a 500.
    if (err instanceof VerifyError) {
      sendJson(res, 401, { error: 'login verification failed' })
      return
    }
    throw err
  }

  const sessionId = randomBytes(24).toString('hex')
  const expiresAt = Math.floor(Date.now() / 1000) + sessionTtl
  sessions.set(sessionId, { did: verified.did, expiresAt })
  clearCookie(res, LEGACY_NONCE_COOKIE)
  setCookie(res, SESSION_COOKIE, sessionId, sessionTtl, isSecureRequest(req))
  const response: PeginSessionResponse = { did: verified.did, sub: verified.did, expiresAt }
  sendJson(res, 200, response)
}

function handleGetSession(req: IncomingMessage, res: ServerResponse): void {
  purgeExpired()
  const session = loadSession(req)
  if (!session) {
    sendJson(res, 401, { error: 'not authenticated' })
    return
  }
  sendJson(res, 200, { did: session.did, sub: session.did, expiresAt: session.expiresAt })
}

function handleLogout(req: IncomingMessage, res: ServerResponse): void {
  const sessionId = readCookie(req, SESSION_COOKIE)
  if (sessionId) sessions.delete(sessionId)
  clearCookie(res, SESSION_COOKIE)
  clearCookie(res, LEGACY_NONCE_COOKIE)
  sendJson(res, 204, null)
}

function loadSession(req: IncomingMessage): StoredSession | null {
  const sessionId = readCookie(req, SESSION_COOKIE)
  if (!sessionId) return null
  const session = sessions.get(sessionId)
  if (!session || session.expiresAt < Math.floor(Date.now() / 1000)) {
    if (sessionId) sessions.delete(sessionId)
    return null
  }
  return session
}

function audienceFromRequest(req: IncomingMessage): string {
  const origin = headerValue(req, 'origin')
  if (origin) return origin
  const host = headerValue(req, 'host')
  if (!host) throw new Error('cannot determine site origin for JWT aud')
  const proto = headerValue(req, 'x-forwarded-proto') ?? 'http'
  return `${proto}://${host}`
}

function headerValue(req: IncomingMessage, name: string): string | undefined {
  const raw = req.headers[name.toLowerCase()]
  if (Array.isArray(raw)) return raw[0]
  return raw
}

function purgeExpired(): void {
  const now = Date.now()
  const nowSec = Math.floor(now / 1000)
  for (const [id, pending] of pendingLogins) {
    if (pending.expiresAt < now) pendingLogins.delete(id)
  }
  for (const [id, session] of sessions) {
    if (session.expiresAt < nowSec) sessions.delete(id)
  }
}

async function readJsonBody<T>(req: IncomingMessage): Promise<T> {
  const chunks: Buffer[] = []
  for await (const chunk of req) {
    chunks.push(typeof chunk === 'string' ? Buffer.from(chunk) : (chunk as Buffer))
  }
  const text = Buffer.concat(chunks).toString('utf8')
  if (!text) throw new Error('request body required')
  return JSON.parse(text) as T
}

function sendJson(res: ServerResponse, status: number, body: unknown): void {
  if (status === 204) {
    res.statusCode = 204
    res.end()
    return
  }
  res.statusCode = status
  res.setHeader('content-type', 'application/json; charset=utf-8')
  res.end(JSON.stringify(body))
}

function pushCookie(res: ServerResponse, value: string): void {
  if (typeof res.appendHeader === 'function') {
    res.appendHeader('set-cookie', value)
    return
  }
  const existing = res.getHeader('set-cookie')
  if (!existing) {
    res.setHeader('set-cookie', value)
  } else if (Array.isArray(existing)) {
    res.setHeader('set-cookie', [...existing.map(String), value])
  } else {
    res.setHeader('set-cookie', [String(existing), value])
  }
}

function setCookie(
  res: ServerResponse,
  name: string,
  value: string,
  maxAgeSeconds: number,
  secure: boolean,
): void {
  const flags = `Path=/; HttpOnly; SameSite=Strict; Max-Age=${maxAgeSeconds}`
  pushCookie(res, `${name}=${value}; ${flags}${secure ? '; Secure' : ''}`)
}

/** True when the request reached us over TLS (direct or via a trusted proxy). */
function isSecureRequest(req: IncomingMessage): boolean {
  if (headerValue(req, 'x-forwarded-proto') === 'https') return true
  return (req.socket as { encrypted?: boolean }).encrypted === true
}

function clearCookie(res: ServerResponse, name: string): void {
  pushCookie(
    res,
    `${name}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT`,
  )
}

function readCookie(req: IncomingMessage, name: string): string | undefined {
  const header = headerValue(req, 'cookie')
  if (!header) return undefined
  for (const part of header.split(';')) {
    const [key, ...rest] = part.trim().split('=')
    if (key === name) return rest.join('=')
  }
  return undefined
}
