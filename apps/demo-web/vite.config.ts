import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
// Plain ESM plugin — loaded at dev/preview time only.
// @ts-expect-error no TS declaration merge for .mjs in this repo layout
import { peginAuthPlugin } from './plugins/pegin-auth-plugin.mjs'

const AUTH_PORT = process.env.PEGIN_AUTH_PORT ?? '8787'

// Forward /api/pegin/* to the Rust auth sidecar (prefix stripped to match its routes).
const authProxy = {
  '/api/pegin': {
    target: `http://127.0.0.1:${AUTH_PORT}`,
    changeOrigin: false,
    rewrite: (path: string) => path.replace(/^\/api\/pegin/, ''),
  },
}

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait(), peginAuthPlugin()],
  server: { proxy: authProxy },
  preview: { proxy: authProxy },
  resolve: {
    alias: {
      // Gitignored web-target artifact — build with `pnpm build:wasm` (workspace root).
      '@pegin/wasm': fileURLToPath(
        new URL('../../packages/sdk/wasm/pegin_wasm.js', import.meta.url),
      ),
    },
  },
})
