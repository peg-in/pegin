// Loads the gitignored WASM build artifact (pnpm build:wasm). `wasm` is
// undefined when it has not been built, so suites skip instead of fail and
// `pnpm -r test` works on a fresh clone.
import { existsSync, readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import {
  alternateTestPhrase,
  deterministicTestPhrase,
  DETERMINISTIC_DID_PK,
  DETERMINISTIC_WALLET_PK,
} from './deterministic-phrase.js'

const WASM_ENTRY = new URL('../../wasm/pegin_wasm.js', import.meta.url)
const WASM_BINARY = new URL('../../wasm/pegin_wasm_bg.wasm', import.meta.url)

export const wasmBuilt =
  existsSync(fileURLToPath(WASM_ENTRY)) && existsSync(fileURLToPath(WASM_BINARY))

async function loadWasm() {
  const mod = await import(WASM_ENTRY.href)
  const bytes = readFileSync(fileURLToPath(WASM_BINARY))
  await mod.default(bytes)
  return mod
}

export const wasm = wasmBuilt ? await loadWasm() : undefined

/** Deterministic 24-word phrase from zero entropy — no English words in source. */
export function testMnemonic(): string {
  return deterministicTestPhrase()
}

export { alternateTestPhrase, DETERMINISTIC_DID_PK, DETERMINISTIC_WALLET_PK }
