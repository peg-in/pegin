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

/** Same-origin PEGIN auth API (`/api/pegin` by default). */
export class PeginAuthClient {
  constructor(private readonly apiPrefix = '/api/pegin') {}

  async requestNonce(): Promise<PeginNoncePayload> {
    const res = await fetch(`${this.apiPrefix}/nonce`, {
      method: 'POST',
      credentials: 'include',
    })
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
    const res = await fetch(`${this.apiPrefix}/session`, {
      method: 'POST',
      credentials: 'include',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(body),
    })
    if (!res.ok) {
      throw new Error(await readError(res, 'login verification failed'))
    }
    return res.json() as Promise<PeginServerSession>
  }

  async getSession(): Promise<PeginServerSession | null> {
    const res = await fetch(`${this.apiPrefix}/session`, { credentials: 'include' })
    if (res.status === 401) return null
    if (!res.ok) {
      throw new Error(await readError(res, 'failed to load session'))
    }
    return res.json() as Promise<PeginServerSession>
  }

  async logout(): Promise<void> {
    const res = await fetch(`${this.apiPrefix}/logout`, {
      method: 'POST',
      credentials: 'include',
    })
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
