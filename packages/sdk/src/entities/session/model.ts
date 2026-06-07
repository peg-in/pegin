// PeginSession — flat, complete record. No optional fields.
// DOD rule: every valid state has a named type. No Option<Option<T>>.

export interface PeginSession {
  /** Stable user identifier (JWT sub claim, derived from DID). */
  sub: string;
  /** The user's Chia DID — `did:chia:<launcher-id>`. */
  did: string;
  /** Raw JWT string issued by the PEGIN backend. */
  jwt: string;
  /** Unix timestamp (seconds) when the JWT expires. */
  expiresAt: number;
  /** Human-readable display name (preferred_username in JWT). */
  username: string;
}
