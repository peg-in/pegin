/** Maps Rust/IPC errors to short user-facing copy. */
export function errorText(err: unknown): string {
  const msg = typeof err === 'string' ? err : err instanceof Error ? err.message : String(err)
  if (msg.includes('invalid seed')) return 'that seed phrase is not valid (bip-39 check failed)'
  if (msg.includes('wrong password')) return 'could not unlock — wrong device key or legacy vault'
  if (msg.includes('No matching entry') || msg.includes('could not be verified')) {
    return 'device unlock key missing — re-import your seed on the import tab'
  }
  return msg
}
