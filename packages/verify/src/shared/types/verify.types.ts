/** Inputs for full login verification on a relying party. */
export interface VerifyLoginInput {
  jwt: string
  expectedAud: string
  challengeNonce?: string
  challengeSigHex?: string
  /** When set, checks that the DID launcher exists on coinset. */
  coinsetBaseUrl?: string
  now?: number
}

/** Successful verification result. */
export interface VerifiedLogin {
  did: string
  aud: string
  didPkHex: string
  nonce?: string
}

export class VerifyError extends Error {
  constructor(message: string) {
    super(message)
    this.name = 'VerifyError'
  }
}
