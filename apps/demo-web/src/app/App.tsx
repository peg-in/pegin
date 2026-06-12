// App shell — header with theme toggle; the login page handles its own WASM gating.

import { LoginPage } from '../pages/login/index.js'
import { ThemeToggle } from '../shared/ui/ThemeToggle.js'

export function App() {
  return (
    <div className="shell">
      <header className="app-header">
        <h1 className="brand">
          <span aria-hidden="true">🐧</span> PEGIN Demo
        </h1>
        <ThemeToggle />
      </header>
      <p className="tagline">
        Decentralized SSO on Chia — any testnet seed phrase whose wallet has an on-chain DID.
      </p>
      <LoginPage />
    </div>
  )
}
