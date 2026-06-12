declare module '@pegin/wasm' {
  export default function init(): Promise<void>
  export function loginWithSeed(
    mnemonic: string,
    peer_url: string | null | undefined,
    ttl_seconds: number,
    aud: string,
    challenge_nonce?: string | null,
  ): Promise<{ did: string; jwt: string; challengeSig?: string }>
}
