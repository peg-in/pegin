/// <reference types="vite/client" />

interface ImportMetaEnv {
  /** Login mode; `demo` exposes the seed → passkey enrollment screen (dev / CI only). */
  readonly PEGIN_LOGIN_MODE?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
