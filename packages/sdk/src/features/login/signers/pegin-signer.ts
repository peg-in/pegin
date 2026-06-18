/**
 * The client signer seam (feat-37). `loginWithPegin` never knows *how* a login is signed —
 * it resolves the account key on the relay, then asks the signer to prove possession.
 * Production logins use PasskeySigner; SeedSigner is test-only. No secret leaves the signer.
 */

/** The login challenge the relay issues for the resolved owner. */
export interface SignLoginRequest {
  /** Canonical `did:chia` the relay resolved from the account key. */
  did: string
  /** DID-owning address index the relay resolved. */
  ownerIndex: number
  /** Relying-party origin to bind into the JWT. */
  aud: string
  /** Server nonce to sign for replay resistance. */
  nonce: string
}

/** What a signer returns: a minted JWT and (when challenged) a BLS challenge signature. */
export interface SignedLogin {
  jwt: string
  challengeSig?: string
}

/** Watch-only identity the relay resolves to `{ did, ownerIndex }`. */
export interface IdentityKey {
  accountPk: string
}

/** Swap point for how a login is signed; `loginWithPegin` is agnostic to the impl. */
export interface PeginSigner {
  /** The account key the relay's `/resolve` maps to a DID + owner index. */
  identityKey(): Promise<IdentityKey>
  /** Prove possession: mint the JWT + sign the challenge for the resolved owner. */
  signLogin(req: SignLoginRequest): Promise<SignedLogin>
}
