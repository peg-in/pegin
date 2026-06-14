// Ambient types for the gitignored wasm-pack artifact (runtime path aliased in vite.config.ts).
declare module '@pegin/wasm' {
  /** Initialises the WASM engine; idempotent across calls. */
  export default function init(): Promise<void>
}
