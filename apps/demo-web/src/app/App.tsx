// App shell — full-viewport ratatui-style terminal frame.

import { LoginPage } from '../pages/login/index.js'
import { ThemeToggle } from '../shared/ui/ThemeToggle.js'

export function App() {
  return (
    <div className="tui-root">
      <div className="tui-screen">
        <header className="tui-titlebar">
          <span>
            <span className="tui-titlebar-mark">┌</span> pegin-demo
          </span>
          <span>testnet11</span>
        </header>
        <main className="tui-body">
          <LoginPage />
        </main>
        <footer className="tui-statusbar">
          <span className="tui-statusbar-hint">server session · chia did</span>
          <ThemeToggle />
        </footer>
      </div>
    </div>
  )
}
