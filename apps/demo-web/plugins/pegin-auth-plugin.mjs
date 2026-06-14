/**
 * Spawns the Rust PEGIN auth backend (crates/pegin-verify `pegin-auth-server`) as a sidecar.
 * Vite proxies `/api/pegin/*` to it (see vite.config.ts). The Rust server is the single source
 * of truth: it verifies the JWT, challenge signature, and on-chain DID **ownership**.
 */

import { execFileSync, spawn } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '../../..')
const BIN = join(ROOT, 'target/debug/pegin-auth-server')
const PORT = process.env.PEGIN_AUTH_PORT ?? '8787'

/** @returns {import('vite').Plugin} */
export function peginAuthPlugin() {
  /** @type {import('node:child_process').ChildProcess | undefined} */
  let server

  const start = () => {
    if (server) return
    process.stdout.write('building pegin-auth-server (cargo)…\n')
    execFileSync('cargo', ['build', '-p', 'pegin-verify', '--bin', 'pegin-auth-server'], {
      cwd: ROOT,
      stdio: 'inherit',
    })
    server = spawn(BIN, [], {
      cwd: ROOT,
      env: {
        ...process.env,
        PEGIN_AUTH_PORT: PORT,
        PEGIN_COINSET_URL: process.env.PEGIN_COINSET_URL ?? 'https://testnet11.api.coinset.org',
      },
      stdio: 'inherit',
    })
    const stop = () => {
      server?.kill()
      server = undefined
    }
    server.on('exit', () => {
      server = undefined
    })
    process.once('exit', stop)
    process.once('SIGINT', () => {
      stop()
      process.exit(0)
    })
    process.once('SIGTERM', stop)
  }

  return {
    name: 'pegin-auth-sidecar',
    configureServer: start,
    configurePreviewServer: start,
  }
}
