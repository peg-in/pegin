#!/usr/bin/env node
/**
 * Ensures WASM is up to date, then runs the given command when the artifact exists.
 */
import { execSync } from 'node:child_process'
import { existsSync } from 'node:fs'
import { fileURLToPath } from 'node:url'

const wasmEntry = fileURLToPath(new URL('../../../packages/sdk/wasm/pegin_wasm.js', import.meta.url))

execSync('node scripts/ensure-wasm.mjs', {
  cwd: fileURLToPath(new URL('..', import.meta.url)),
  stdio: 'inherit',
})

if (!existsSync(wasmEntry)) {
  process.stdout.write('demo-web skipped: packages/sdk/wasm not built — run `pnpm build:wasm` first\n')
  process.exit(0)
}

try {
  execSync(process.argv.slice(2).join(' '), { stdio: 'inherit' })
} catch (error) {
  process.exit(typeof error.status === 'number' ? error.status : 1)
}
