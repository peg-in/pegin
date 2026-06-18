/**
 * Passkey-guarded seed vault (feat-37, model B).
 *
 * The seed is encrypted under a key derived from the WebAuthn PRF secret (AES-256-GCM) and
 * the ciphertext is stored locally. A passkey assertion reproduces the PRF secret, which
 * decrypts the seed — so the passkey *guards* an existing wallet/DID rather than minting a
 * fresh one. The plaintext seed exists only transiently in memory during enroll/login.
 *
 * Storage is local (one vault per browser). Multi-device sync of the ciphertext (DID profile
 * or relay) is feat-18; the synced *passkey* still reproduces the PRF secret on any device.
 */

import { randomBytes } from '../../../shared/lib/random.js'

/** Encrypted seed at rest: AES-GCM IV + ciphertext, both base64. */
export interface VaultBlob {
  iv: string
  ct: string
}

/** Where the encrypted seed lives. Injectable so tests use an in-memory store. */
export interface PasskeyVaultStore {
  load(): VaultBlob | null
  save(blob: VaultBlob): void
  clear(): void
}

const DEFAULT_KEY = 'pegin.passkey.vault'

/** Browser `localStorage`-backed vault store (the demo default). */
export function localStorageVault(key = DEFAULT_KEY): PasskeyVaultStore {
  return {
    load: () => {
      try {
        const raw = localStorage.getItem(key)
        return raw ? (JSON.parse(raw) as VaultBlob) : null
      } catch {
        // Corrupt JSON or storage access denied → treat as not enrolled.
        return null
      }
    },
    save: (blob) => {
      localStorage.setItem(key, JSON.stringify(blob))
    },
    clear: () => {
      localStorage.removeItem(key)
    },
  }
}

/** True once a seed has been sealed under a passkey on this device (a vault blob exists). */
export function isPasskeyEnrolled(vault: PasskeyVaultStore = localStorageVault()): boolean {
  return vault.load() !== null
}

/** Loads PRF-encrypted seed ciphertext synced from PEGIN Signer via the auth relay. */
export async function fetchPasskeyBlobFromRelay(
  credentialId: string,
  apiPrefix = '/api/pegin',
): Promise<VaultBlob | null> {
  const res = await fetch(
    `${apiPrefix}/passkey-blob?credentialId=${encodeURIComponent(credentialId)}`,
    { credentials: 'include', signal: AbortSignal.timeout(15_000) },
  )
  if (res.status === 404) return null
  if (!res.ok) {
    throw new Error(`passkey blob fetch failed (${res.status})`)
  }
  return res.json() as Promise<VaultBlob>
}

/** Encrypts `mnemonic` under the PRF secret. */
export async function encryptSeed(prfSecret: Uint8Array, mnemonic: string): Promise<VaultBlob> {
  const key = await importAesKey(prfSecret)
  const iv = randomBytes(12)
  const ct = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, utf8(mnemonic))
  return { iv: toBase64(iv), ct: toBase64(new Uint8Array(ct)) }
}

/** Recovers the mnemonic from `blob` using the PRF secret. Throws on a wrong secret. */
export async function decryptSeed(prfSecret: Uint8Array, blob: VaultBlob): Promise<string> {
  const key = await importAesKey(prfSecret)
  try {
    const pt = await crypto.subtle.decrypt(
      { name: 'AES-GCM', iv: fromBase64(blob.iv) },
      key,
      fromBase64(blob.ct),
    )
    return new TextDecoder().decode(pt)
  } catch {
    throw new Error(
      'passkey decrypt failed — pick the passkey registered in PEGIN Signer (localhost) and re-sync if needed',
    )
  }
}

// The PRF secret is already a KDF output, so use its first 32 bytes directly as the AES key.
async function importAesKey(secret: Uint8Array): Promise<CryptoKey> {
  return crypto.subtle.importKey('raw', toArrayBuffer(secret.subarray(0, 32)), 'AES-GCM', false, [
    'encrypt',
    'decrypt',
  ])
}

// Helpers below pin ArrayBuffer-backed views so they satisfy WebCrypto's `BufferSource`.

function utf8(text: string): Uint8Array<ArrayBuffer> {
  return toArrayBuffer(new TextEncoder().encode(text))
}

function toArrayBuffer(src: Uint8Array): Uint8Array<ArrayBuffer> {
  const out = new Uint8Array(new ArrayBuffer(src.length))
  out.set(src)
  return out
}

function toBase64(bytes: Uint8Array): string {
  let binary = ''
  for (const byte of bytes) binary += String.fromCharCode(byte)
  return btoa(binary)
}

function fromBase64(value: string): Uint8Array<ArrayBuffer> {
  const binary = atob(value)
  const out = new Uint8Array(new ArrayBuffer(binary.length))
  for (let i = 0; i < binary.length; i += 1) out[i] = binary.charCodeAt(i)
  return out
}
