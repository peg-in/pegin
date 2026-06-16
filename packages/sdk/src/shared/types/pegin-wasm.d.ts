declare module '@pegin/wasm' {
  export default function init(): Promise<void>

  export interface WalletKeys {
    readonly didPkHex: string
    readonly walletPkHex: string
    readonly didPublicKey: Uint8Array
    /** Watch-only observer account key (48-byte BLS hex) the relay resolves to a DID. */
    readonly accountPkHex: string
    free(): void
  }

  export function deriveWalletKeys(mnemonic: string): WalletKeys
  export function deriveKeys(mnemonic: string): WalletKeys

  /** Watch-only identity the relay maps to `{ did, ownerIndex }`. No chain I/O here. */
  export function identityKey(keys: WalletKeys): { accountPk: string }

  /**
   * Mints a JWT + signs the login challenge for the relay-resolved owner. Secrets stay
   * in WASM; only the JWT and signature are returned.
   * @param did - canonical `did:chia` from `/resolve`
   * @param ownerIndex - DID-owning address index from `/resolve`
   * @param aud - relying-party origin bound into the JWT
   * @param ttlSeconds - JWT lifetime
   * @param nonce - server challenge; when set the result carries `challengeSig`
   */
  export function signLogin(
    keys: WalletKeys,
    did: string,
    ownerIndex: number,
    aud: string,
    ttlSeconds: number,
    nonce?: string | null,
  ): { jwt: string; challengeSig?: string }
}
