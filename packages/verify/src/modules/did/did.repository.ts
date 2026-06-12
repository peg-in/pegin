import { bech32m } from 'bech32'

const DID_HRP = 'did:chia:'

/** Parses a `did:chia:…` bech32m string into lowercase launcher-id hex. */
export function launcherIdHexFromDid(did: string): string {
  if (!did.startsWith(DID_HRP)) {
    throw new Error(`expected bech32m '${DID_HRP}1…', got '${did}'`)
  }
  const decoded = bech32m.decode(did)
  if (decoded.prefix !== DID_HRP) {
    throw new Error(`unexpected DID prefix '${decoded.prefix}'`)
  }
  const bytes = Uint8Array.from(bech32m.fromWords(decoded.words))
  if (bytes.length !== 32) {
    throw new Error(`DID launcher ID must be 32 bytes, got ${bytes.length}`)
  }
  return bytesToHex(bytes)
}

/** Returns true when the launcher coin record exists on coinset (spent or unspent). */
export async function launcherExists(baseUrl: string, launcherHex: string): Promise<boolean> {
  const url = `${baseUrl.replace(/\/$/, '')}/get_coin_record_by_name`
  const resp = await fetch(url, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ name: `0x${launcherHex}` }),
  })
  if (!resp.ok) {
    throw new Error(`coinset HTTP ${resp.status}`)
  }
  const body = (await resp.json()) as {
    success?: boolean
    error?: string
    coin_record?: unknown
  }
  if (!body.success) {
    const msg = body.error ?? 'unknown error'
    if (msg.includes('not found')) return false
    throw new Error(`coinset: ${msg}`)
  }
  return body.coin_record != null
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('')
}
