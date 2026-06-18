/** WebAuthn rpId — map dev loopback hosts to `localhost` so Signer and demo-web share passkeys. */
export function normalizeWebAuthnRpId(hostname: string): string {
  const host = hostname.trim().toLowerCase()
  if (host === '127.0.0.1' || host === '[::1]' || host === '0:0:0:0:0:0:0:1') {
    return 'localhost'
  }
  return host
}
