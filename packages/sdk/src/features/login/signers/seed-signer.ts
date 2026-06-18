/**
 * **Test-only** signer: derives BLS keys from a mnemonic in WASM and signs locally.
 *
 * Not exported from the public SDK API — `PasskeySigner` is the login path (it seals the
 * seed under a passkey at enrollment). A standalone browser seed field is a phishing
 * surface (feat-37). Unit tests construct this directly; recovery is feat-23.
 */

import type { IdentityKey, PeginSigner, SignLoginRequest, SignedLogin } from './pegin-signer.js'

/** Opaque WASM wallet-keys handle (secret scalars zeroized on `free()`). */
export interface WasmWalletKeys {
  readonly accountPkHex: string
  free(): void
}

/** The WASM surface SeedSigner depends on — injected so tests can stub it. */
export interface SeedSignerWasm {
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

export interface SeedSignerOptions {
  /** JWT lifetime in seconds. Default 3600. */
  jwtTtlSeconds?: number
  /** Loads the browser WASM module (must alias `@pegin/wasm` in the bundler). */
  loadWasm?: () => Promise<SeedSignerWasm>
}

export class SeedSigner implements PeginSigner {
  private keysPromise: Promise<{ wasm: SeedSignerWasm; keys: WasmWalletKeys }> | undefined
  private readonly ttl: number

  constructor(
    private readonly mnemonic: string,
    private readonly options: SeedSignerOptions = {},
  ) {
    this.ttl = options.jwtTtlSeconds ?? 3600
  }

  // Derive once; identityKey and signLogin share the same WASM key handle.
  private resolveKeys(): Promise<{ wasm: SeedSignerWasm; keys: WasmWalletKeys }> {
    this.keysPromise ??= (async () => {
      const wasm = await (this.options.loadWasm ?? defaultLoadWasm)()
      return { wasm, keys: wasm.deriveWalletKeys(this.mnemonic) }
    })()
    return this.keysPromise
  }

  async identityKey(): Promise<IdentityKey> {
    const { wasm, keys } = await this.resolveKeys()
    return wasm.identityKey(keys)
  }

  async signLogin(req: SignLoginRequest): Promise<SignedLogin> {
    const { wasm, keys } = await this.resolveKeys()
    return wasm.signLogin(keys, req.did, req.ownerIndex, req.aud, this.ttl, req.nonce)
  }

  /** Releases the WASM key handle (zeroizes secrets). Call once the login completes. */
  async dispose(): Promise<void> {
    const resolved = await this.keysPromise
    resolved?.keys.free()
    this.keysPromise = undefined
  }
}

async function defaultLoadWasm(): Promise<SeedSignerWasm> {
  const mod = await import('@pegin/wasm')
  await mod.default()
  return mod
}
