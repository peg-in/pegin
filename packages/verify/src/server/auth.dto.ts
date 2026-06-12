/** Session shapes for the PEGIN auth handler. */
export interface PeginAuthSession {
  did: string
  sub: string
  expiresAt: number
}

export interface PeginNonceResponse {
  /** Opaque id — send back with `POST /session` to bind this login attempt. */
  loginId: string
  nonce: string
  aud: string
}

export interface PeginSessionResponse {
  did: string
  sub: string
  expiresAt: number
}

export interface PeginCompleteLoginBody {
  loginId: string
  jwt: string
  challengeSig?: string
}
