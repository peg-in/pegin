#!/usr/bin/env node
/**
 * Interactive launcher for the PEGIN demos — run via `pnpm dev`.
 * Pass a name to skip the prompt: `pnpm dev web` | `mini` | `cli`.
 * Each choice just delegates to the matching `dev:<key>` package script.
 */
import { spawn } from 'node:child_process'
import { createInterface } from 'node:readline'

const demos = [
  { key: 'web', label: 'web  — browser Login with PEGIN (Vite)', script: 'dev:web' },
  { key: 'mini', label: 'mini — Tauri desktop wallet (skeleton, not runnable yet — feat-5)', script: 'dev:mini' },
  { key: 'cli', label: 'cli  — Node CLI demo (mnemonic → DID → JWT)', script: 'dev:cli' },
]

/** Spawns `pnpm run <script>` with inherited stdio and mirrors its exit code. */
function run(script) {
  const child = spawn('pnpm', ['run', script], { stdio: 'inherit' })
  child.on('error', (err) => {
    process.stderr.write(`failed to launch pnpm: ${err.message}\n`)
    process.exit(1)
  })
  child.on('exit', (code) => process.exit(code ?? 0))
}

/** Resolves an answer (1-based number or key) to a demo, or undefined. */
function pick(answer) {
  const trimmed = answer.trim()
  return demos[Number.parseInt(trimmed, 10) - 1] ?? demos.find((d) => d.key === trimmed)
}

const requested = process.argv[2]
if (requested) {
  const demo = pick(requested)
  if (!demo) {
    process.stderr.write(`Unknown demo '${requested}'. Choose: ${demos.map((d) => d.key).join(', ')}\n`)
    process.exit(1)
  }
  run(demo.script)
} else {
  const menu = ['PEGIN demos:', ...demos.map((d, i) => `  ${i + 1}) ${d.label}`), ''].join('\n')
  process.stdout.write(`${menu}\n`)
  const rl = createInterface({ input: process.stdin, output: process.stdout })
  rl.question(`Select [1-${demos.length}]: `, (answer) => {
    rl.close()
    const demo = pick(answer)
    if (!demo) {
      process.stderr.write('No demo selected.\n')
      process.exit(1)
    }
    run(demo.script)
  })
}
