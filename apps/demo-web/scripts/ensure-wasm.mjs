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
const CRATE_SRC = join(ROOT, 'crates/pegin-wasm/src')
const WASM_BIN = join(ROOT, 'packages/sdk/wasm/pegin_wasm_bg.wasm')

function newestMtimeMs(dir) {
  let newest = 0
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name)
    if (entry.isDirectory()) {
      newest = Math.max(newest, newestMtimeMs(path))
    } else if (entry.isFile()) {
      newest = Math.max(newest, statSync(path).mtimeMs)
    }
  }
  return newest
}

function needsRebuild() {
  if (!existsSync(WASM_BIN)) return true
  const wasmMtime = statSync(WASM_BIN).mtimeMs
  return newestMtimeMs(CRATE_SRC) > wasmMtime
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
