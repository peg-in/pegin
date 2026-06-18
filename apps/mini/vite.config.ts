import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// Tauri drives this dev server (see src-tauri/tauri.conf.json devUrl/beforeDevCommand).
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
})
