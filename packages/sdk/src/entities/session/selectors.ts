// Pure selectors — data in, derived data out. No side effects.
// DOD rule: derived state is computed here, never stored alongside its source.

import type { PeginSession } from "./model.js";

export function selectIsExpired(session: PeginSession): boolean {
  return Date.now() / 1000 > session.expiresAt;
}

export function selectDisplayName(session: PeginSession | null): string {
  return session?.username ?? "Anonymous";
}

export function selectDid(session: PeginSession | null): string | null {
  return session?.did ?? null;
}
