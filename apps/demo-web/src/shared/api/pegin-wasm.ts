// Typed loader for the PEGIN browser mini wallet (@pegin/wasm, vite-aliased artifact).

/** Plain object returned by `loginWithSeed` — no WASM class wrapper. */
export interface LoginSession {
  did: string
  jwt: string
}

type WasmModule = typeof import('@pegin/wasm')
export type PeginWasm = Omit<WasmModule, 'loginWithSeed' | 'default'> & {
  loginWithSeed(
    mnemonic: string,
    peer_url: string | null | undefined,
    ttl_seconds: number,
  ): Promise<LoginSession>
}
export type { WalletKeys } from '@pegin/wasm'

async function initWasmModule(): Promise<PeginWasm> {
  const mod = await import('@pegin/wasm')
  await mod.default()
  return mod as PeginWasm
}

// Import starts at module evaluation so the WASM downloads and compiles in
// parallel with React startup instead of after the user's first interaction.
const wasmPromise: Promise<PeginWasm> = initWasmModule()

// Pre-attach a handler — a load failure is surfaced by the UI awaiting
// loadPeginWasm(), not as an unhandled rejection before React mounts.
void wasmPromise.catch(() => undefined)

export function loadPeginWasm(): Promise<PeginWasm> {
  return wasmPromise
}
