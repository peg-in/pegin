declare module '@pegin/wasm' {
  export default function init(): Promise<void>

  export interface WalletKeys {
    readonly didPkHex: string
    readonly walletPkHex: string
    readonly didPublicKey: Uint8Array
    free(): void
  }

  export function deriveWalletKeys(mnemonic: string): WalletKeys
  export function deriveKeys(mnemonic: string): WalletKeys

  export interface DidIdentity {
    ownerIndex: number
    ownerPk: string
    did?: string
  }

  /**
   * Resolves login identity from wallet keys. Cache hit is instant; a first-login
   * miss scans public coinset hints (no secrets leave the browser).
   * @param keys - wallet keys used for identity resolution
   * @param scan_limit - highest address index to probe; 0 uses the default (10 000)
   */
  export function lookupDid(keys: WalletKeys, scan_limit: number): Promise<DidIdentity>

  export function loginWithSeed(
    mnemonic: string,
    scan_limit: number,
    ttl_seconds: number,
    aud: string,
    challenge_nonce?: string | null,
  ): Promise<{
    did: string
    jwt: string
    challengeSig?: string
    walletFp: string
    ownerIndex: number
  }>
  export function rememberDid(walletFp: string, did: string, ownerIndex: number): void
}
