// DID string helpers and type guard.
// A DID is a plain string; these helpers provide validation and parsing.

const DID_PREFIX = 'did:chia:' as const

export type ChiaDid = `did:chia:${string}`

export function isChiaDid(value: string): value is ChiaDid {
  return value.startsWith(DID_PREFIX) && value.length > DID_PREFIX.length
}

export function launcherIdFromDid(did: ChiaDid): string {
  return did.slice(DID_PREFIX.length)
}

export function didFromLauncherId(launcherId: string): ChiaDid {
  return `${DID_PREFIX}${launcherId}`
}
