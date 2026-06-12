// Cookie-backed demo session — the JWT is the session; the DID comes from its claims.

import { decodeJwtPayload, isJwtExpired } from '@pegin/sdk'

const COOKIE_NAME = 'pegin_jwt'

export interface DemoSession {
  did: string
  jwt: string
}

/** Persists the JWT for exactly its remaining lifetime — the browser logs us out at `exp`. */
export function saveSession(jwt: string): void {
  const exp = decodeJwtPayload(jwt)['exp']
  if (typeof exp !== 'number') return
  const maxAge = Math.max(0, Math.floor(exp - Date.now() / 1000))
  const secure = location.protocol === 'https:' ? '; Secure' : ''
  document.cookie = `${COOKIE_NAME}=${jwt}; Max-Age=${maxAge}; Path=/; SameSite=Strict${secure}`
}

/** Restores the session from the cookie; expired or malformed JWTs yield `null`. */
export function loadSession(): DemoSession | null {
  const jwt = document.cookie
    .split('; ')
    .find((entry) => entry.startsWith(`${COOKIE_NAME}=`))
    ?.slice(COOKIE_NAME.length + 1)
  if (!jwt) return null
  try {
    if (isJwtExpired(jwt)) return null
    const did = decodeJwtPayload(jwt)['iss']
    return typeof did === 'string' ? { did, jwt } : null
  } catch {
    return null
  }
}

export function clearSession(): void {
  document.cookie = `${COOKIE_NAME}=; Max-Age=0; Path=/; SameSite=Strict`
}
