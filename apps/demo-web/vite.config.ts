import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      // Gitignored web-target artifact — build with `pnpm build:wasm` (workspace root).
      '@pegin/wasm': fileURLToPath(
        new URL('../../packages/sdk/wasm/pegin_wasm.js', import.meta.url),
      ),
    },
  },
})
