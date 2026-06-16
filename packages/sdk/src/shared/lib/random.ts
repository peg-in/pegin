/** Cryptographically random bytes in a fresh ArrayBuffer (satisfies WebCrypto `BufferSource`). */
export function randomBytes(length: number): Uint8Array<ArrayBuffer> {
  const buffer = new ArrayBuffer(length)
  crypto.getRandomValues(new Uint8Array(buffer))
  return new Uint8Array(buffer)
}
