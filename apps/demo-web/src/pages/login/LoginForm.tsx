// Passkey sign-in — a single button that opens the browser / 1Password passkey picker.

import type { SubmitEvent } from 'react'

interface LoginFormProps {
  loading: boolean
  wasmReady: boolean
  onAuthenticate: () => void
}

export function LoginForm({ loading, wasmReady, onAuthenticate }: LoginFormProps) {
  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    onAuthenticate()
  }

  const busy = loading || !wasmReady

  return (
    <form onSubmit={handleSubmit}>
      <p className="tui-line">
        <span className="tui-line-dim">auth</span>
        <span className="tui-line-value">passkey · Face ID / Touch ID / 1Password</span>
      </p>
      <button type="submit" className="tui-primary" disabled={busy}>
        {loading ? (
          <>
            <span className="tui-spinner" aria-hidden="true" />
            authenticating
          </>
        ) : wasmReady ? (
          '▶ sign in with passkey'
        ) : (
          <>
            <span className="tui-spinner" aria-hidden="true" />
            loading wasm
          </>
        )}
      </button>
    </form>
  )
}
