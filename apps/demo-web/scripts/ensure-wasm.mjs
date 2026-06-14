#!/usr/bin/env node
/**
 * Rebuilds browser WASM when missing or when Rust sources are newer than the artifact.
 * Keeps demo-web on the latest wallet logic without hardcoding any test wallet data.
 */
import { execSync } from 'node:child_process'
import { existsSync, readdirSync, statSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = fileURLToPath(new URL('../../../', import.meta.url))
const WASM_BIN = join(ROOT, 'packages/sdk/wasm/pegin_wasm_bg.wasm')

// Inputs that change the wasm output: the crate sources plus its build config and
// the pegin-jwt crate it depends on (build.rs there embeds the HKDF salt).
const WATCHED = [
  'crates/pegin-wasm/src',
  'crates/pegin-wasm/Cargo.toml',
  'crates/pegin-jwt/src',
  'crates/pegin-jwt/Cargo.toml',
  'crates/pegin-jwt/build.rs',
].map((p) => join(ROOT, p))

// Stops at the first file newer than `threshold` instead of walking every tree.
function anyNewerThan(path, threshold) {
  if (!existsSync(path)) return false
  const stat = statSync(path)
  if (!stat.isDirectory()) return stat.mtimeMs > threshold
  return readdirSync(path, { withFileTypes: true }).some((entry) =>
    anyNewerThan(join(path, entry.name), threshold),
  )
}

function needsRebuild() {
  if (!existsSync(WASM_BIN)) return true
  const wasmMtime = statSync(WASM_BIN).mtimeMs
  return WATCHED.some((path) => anyNewerThan(path, wasmMtime))
}

function wasmPackAvailable() {
  try {
    execSync('wasm-pack --version', { stdio: 'ignore' })
    return true
  } catch {
    return false
  }
}

if (needsRebuild()) {
  if (!wasmPackAvailable()) {
    process.stdout.write('wasm-pack not installed — skipping browser WASM rebuild\n')
    process.exit(0)
  }
  process.stdout.write('pegin-wasm sources changed — rebuilding browser WASM…\n')
  execSync('pnpm build:wasm', { cwd: ROOT, stdio: 'inherit' })
}
