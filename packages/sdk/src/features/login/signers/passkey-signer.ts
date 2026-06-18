/**
 * PasskeySigner — the default M1 login signer (feat-37, model B: passkey guards the seed).
 *
 * A WebAuthn passkey (Face ID / Touch ID / 1Password / synced keychain) unlocks an existing
 * wallet via the `prf` extension: the authenticator returns a stable per-credential secret
 * that decrypts the locally stored, PRF-encrypted seed. WASM then derives the wallet keys
 * from that seed — so the passkey logs into the wallet's existing on-chain DID.
 *
 * No seed is typed on the login path (it is entered once at enrollment), the plaintext seed
 * lives only transiently in memory, and the on-chain owner binding (feat-17) is unchanged.
 *
 * Security comes from the BLS owner proof, so the relay stays signer-agnostic — it does not
 * verify the WebAuthn assertion. Server-side attestation/assertion verification is feat-18.
 */

import { randomBytes } from '../../../shared/lib/random.js'
import type { IdentityKey, PeginSigner, SignLoginRequest, SignedLogin } from './pegin-signer.js'
import {
  decryptSeed,
  encryptSeed,
  localStorageVault,
  type PasskeyVaultStore,
} from './passkey-vault.js'

/** Opaque WASM wallet-keys handle (secret scalars zeroized on `free()`). */
export interface WasmWalletKeys {
  readonly accountPkHex: string
  free(): void
}

/** The WASM surface PasskeySigner needs — injected so tests can stub it. */
export interface PasskeyWasm {
  deriveWalletKeys(mnemonic: string): WasmWalletKeys
  identityKey(keys: WasmWalletKeys): IdentityKey
  signLogin(
    keys: WasmWalletKeys,
    did: string,
    ownerIndex: number,
    aud: string,
    ttlSeconds: number,
    nonce?: string | null,
  ): SignedLogin
}

/** Minimal slice of `navigator.credentials` — injectable for tests. */
export interface WebAuthnApi {
  create(options: CredentialCreationOptions): Promise<Credential | null>
  get(options: CredentialRequestOptions): Promise<Credential | null>
}

export interface PasskeySignerOptions {
  /** Relying-party id (registrable domain), e.g. `app.example`. */
  rpId: string
  /** JWT lifetime in seconds. Default 3600. */
  jwtTtlSeconds?: number
  /** PRF evaluation salt (domain separation). Default `pegin-prf-v1`. */
  prfSalt?: Uint8Array
  /** Where the PRF-encrypted seed lives. Default `localStorage`. */
  vault?: PasskeyVaultStore
  /** Loads the browser WASM module (must alias `@pegin/wasm` in the bundler). */
  loadWasm?: () => Promise<PasskeyWasm>
  /** WebAuthn API; defaults to `navigator.credentials`. */
  webauthn?: WebAuthnApi
}

export interface EnrollPasskeyOptions extends PasskeySignerOptions {
  /** Account label shown by the authenticator UI. */
  userName: string
  /** The seed phrase to seal under the new passkey (entered once, at enrollment). */
  mnemonic: string
  /** Friendly relying-party name. Default `PEGIN`. */
  rpName?: string
}

const DEFAULT_PRF_SALT = new TextEncoder().encode('pegin-prf-v1')
const PRF_SECRET_BYTES = 32

export class PasskeySigner implements PeginSigner {
  private keysPromise: Promise<{ wasm: PasskeyWasm; keys: WasmWalletKeys }> | undefined
  private readonly ttl: number

  constructor(private readonly options: PasskeySignerOptions) {
    this.ttl = options.jwtTtlSeconds ?? 3600
  }

  async identityKey(): Promise<IdentityKey> {
    const { wasm, keys } = await this.unlock()
    return wasm.identityKey(keys)
  }

  async signLogin(req: SignLoginRequest): Promise<SignedLogin> {
    const { wasm, keys } = await this.unlock()
    return wasm.signLogin(keys, req.did, req.ownerIndex, req.aud, this.ttl, req.nonce)
  }

  /** Releases the WASM key handle (zeroizes secrets). Call once the login completes. */
  async dispose(): Promise<void> {
    const resolved = await this.keysPromise
    resolved?.keys.free()
    this.keysPromise = undefined
  }

  // One biometric prompt per login: assert + decrypt + derive once, then share the handle.
  private unlock(): Promise<{ wasm: PasskeyWasm; keys: WasmWalletKeys }> {
    this.keysPromise ??= (async () => {
      const vault = this.options.vault ?? localStorageVault()
      const blob = vault.load()
      if (!blob) {
        throw new Error('no enrolled passkey on this device — enroll first')
      }
      const webauthn = this.options.webauthn ?? navigatorCredentials()
      const secret = await evaluatePrf(webauthn, this.options.rpId, this.salt())
      const mnemonic = await decryptSeed(secret, blob)
      const wasm = await loadWasmModule(this.options.loadWasm)
      return { wasm, keys: wasm.deriveWalletKeys(mnemonic) }
    })()
    return this.keysPromise
  }

  private salt(): Uint8Array {
    return this.options.prfSalt ?? DEFAULT_PRF_SALT
  }
}

/**
 * Provisions a PRF-capable passkey and seals `mnemonic` under its PRF secret. Run once per
 * device; afterwards `PasskeySigner` logs in with no seed input. The seed is the user's
 * existing wallet recovery phrase, so logins resolve its on-chain DID.
 */
export async function enrollPasskey(options: EnrollPasskeyOptions): Promise<void> {
  const webauthn = options.webauthn ?? navigatorCredentials()
  const salt = options.prfSalt ?? DEFAULT_PRF_SALT

  const credential = await webauthn.create({
    publicKey: {
      challenge: randomBytes(32),
      rp: { id: options.rpId, name: options.rpName ?? 'PEGIN' },
      user: { id: randomBytes(16), name: options.userName, displayName: options.userName },
      pubKeyCredParams: [
        { type: 'public-key', alg: -7 },
        { type: 'public-key', alg: -257 },
      ],
      authenticatorSelection: { residentKey: 'required', userVerification: 'preferred' },
      // Evaluate PRF at creation: Chrome platform authenticators return the secret here,
      // saving a second prompt; others report `enabled` and we read it via an assertion.
      extensions: { prf: { eval: { first: salt } } } as AuthenticationExtensionsClientInputs,
    },
  })
  if (!credential) {
    throw new Error('passkey registration was cancelled')
  }

  const created = extensionResults(credential).prf
  if (created?.enabled === false) {
    throw new Error(
      'this authenticator does not support the WebAuthn PRF extension — use 1Password or a PRF-capable passkey',
    )
  }

  // Prefer the create-time PRF secret; fall back to an assertion when it is not returned there.
  const secret = created?.results?.first
    ? toSecret(created.results.first)
    : await evaluatePrf(webauthn, options.rpId, salt, (credential as PublicKeyCredential).rawId)
  const blob = await encryptSeed(secret, options.mnemonic)
  ;(options.vault ?? localStorageVault()).save(blob)
}

// Asserts the passkey (modal dialog — the browser/1Password passkey picker) and reads the
// PRF secret. The challenge is anti-replay only — the relay verifies the BLS owner proof,
// not this assertion (see the module header). `credentialId` targets enrollment's new
// credential; login omits it (discoverable) so the picker lists all matching passkeys.
async function evaluatePrf(
  webauthn: WebAuthnApi,
  rpId: string,
  salt: Uint8Array,
  credentialId?: ArrayBuffer,
): Promise<Uint8Array> {
  const assertion = await webauthn.get({
    publicKey: {
      challenge: randomBytes(32),
      rpId,
      userVerification: 'preferred',
      ...(credentialId ? { allowCredentials: [{ id: credentialId, type: 'public-key' }] } : {}),
      extensions: { prf: { eval: { first: salt } } } as AuthenticationExtensionsClientInputs,
    },
  })
  return readPrfSecret(assertion)
}

function navigatorCredentials(): WebAuthnApi {
  if (typeof navigator === 'undefined') {
    throw new Error('WebAuthn is not available in this environment')
  }
  return navigator.credentials
}

async function loadWasmModule(
  loadWasm: (() => Promise<PasskeyWasm>) | undefined,
): Promise<PasskeyWasm> {
  if (loadWasm) return loadWasm()
  const mod = await import('@pegin/wasm')
  await mod.default()
  return mod
}

/** PRF outputs are exposed on the credential's client-extension results. */
interface PrfExtensionOutput {
  enabled?: boolean
  results?: { first?: BufferSource }
}

function extensionResults(credential: Credential): { prf?: PrfExtensionOutput } {
  return (credential as PublicKeyCredential).getClientExtensionResults()
}

function readPrfSecret(assertion: Credential | null): Uint8Array {
  if (!assertion) {
    throw new Error('passkey assertion was cancelled')
  }
  const first = extensionResults(assertion).prf?.results?.first
  if (!first) {
    throw new Error('authenticator returned no PRF secret — passkey not PRF-capable')
  }
  return toSecret(first)
}

function toSecret(first: BufferSource): Uint8Array {
  // Respect a view's byteOffset/byteLength — `.buffer` alone would read the whole backing store.
  const secret =
    first instanceof ArrayBuffer
      ? new Uint8Array(first)
      : new Uint8Array(first.buffer, first.byteOffset, first.byteLength)
  if (secret.length < PRF_SECRET_BYTES) {
    throw new Error('PRF secret is too short')
  }
  return secret
}
