// Terminal-style frame — titlebar, body slot, statusbar.

import type { ReactNode } from 'react'
import { ThemeToggle } from '../shared/ui/ThemeToggle.js'

const LOGO = `  ▄▀▀▀▄
  ▀   ▀
 ▐ ▄▄▄▌
  ▀▀▀▀
 p e g i n`

interface ShellProps {
  screenLabel: string
  statusHint?: string
  showLogo?: boolean
  children: ReactNode
}

/** Shared TUI chrome for every Signer screen. */
export function Shell({ screenLabel, statusHint, showLogo, children }: ShellProps) {
  return (
    <div className="tui-root">
      <div className="tui-screen">
        <header className="tui-titlebar">
          <span>
            <span className="tui-titlebar-mark">┌</span> pegin-signer
          </span>
          <span>{screenLabel}</span>
        </header>

        <main className="tui-body">
          {showLogo && <pre className="tui-logo" aria-hidden="true">{LOGO}</pre>}
          {children}
        </main>

        <footer className="tui-statusbar">
          <span className="tui-statusbar-hint">{statusHint ?? 'bip-39 · argon2id · passkey prf'}</span>
          <ThemeToggle />
        </footer>
      </div>
    </div>
  )
}
