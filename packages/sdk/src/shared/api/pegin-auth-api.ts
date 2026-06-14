/** Server session returned after verified login. */
export interface PeginServerSession {
  did: string
  sub: string
  expiresAt: number
}

export interface PeginNoncePayload {
  loginId: string
  nonce: string
  aud: string
}

/** Abort a stalled auth request rather than hang the login flow. */
const REQUEST_TIMEOUT_MS = 15_000

/** Same-origin PEGIN auth API (`/api/pegin` by default). */
export class PeginAuthClient {
  constructor(private readonly apiPrefix = '/api/pegin') {}

  /** Same-origin fetch with credentials and a timeout applied to every call. */
  private request(path: string, init: RequestInit = {}): Promise<Response> {
    return fetch(`${this.apiPrefix}${path}`, {
      credentials: 'include',
      signal: AbortSignal.timeout(REQUEST_TIMEOUT_MS),
      ...init,
    })
  }

  async requestNonce(): Promise<PeginNoncePayload> {
    const res = await this.request('/nonce', { method: 'POST' })
    if (!res.ok) {
      throw new Error(await readError(res, 'failed to start login'))
    }
    return res.json() as Promise<PeginNoncePayload>
  }

  async completeLogin(body: {
    loginId: string
    jwt: string
    challengeSig?: string
  }): Promise<PeginServerSession> {
    const res = await this.request('/session', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(body),
    })
    if (!res.ok) {
      throw new Error(await readError(res, 'login verification failed'))
    }
    return res.json() as Promise<PeginServerSession>
  }

  async getSession(): Promise<PeginServerSession | null> {
    const res = await this.request('/session')
    if (res.status === 401) return null
    if (!res.ok) {
      throw new Error(await readError(res, 'failed to load session'))
    }
    return res.json() as Promise<PeginServerSession>
  }

  async logout(): Promise<void> {
    const res = await this.request('/logout', { method: 'POST' })
    if (!res.ok && res.status !== 204) {
      throw new Error(await readError(res, 'logout failed'))
    }
  }
}

async function readError(res: Response, fallback: string): Promise<string> {
  try {
    const body = (await res.json()) as { error?: string }
    return body.error ?? fallback
  } catch {
    return fallback
  }
}
