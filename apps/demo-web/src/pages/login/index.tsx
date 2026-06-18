// Login page — two views, one at a time. Sign-in (existing passkey) is primary; a link below
// switches to the create-passkey view (seal a seed under a new passkey) and back.

import { useState } from 'react'
import { seedEnrollEnabled } from '../../shared/lib/index.js'
import { LoginForm } from './LoginForm.js'
import { LoginResult } from './LoginResult.js'
import { SeedInputForm } from './SeedInputForm.js'
import { SignerRequestPanel } from './SignerRequestPanel.js'
import { useLogin } from './useLogin.js'
import { useWasm } from './useWasm.js'

function truncateDid(did: string): string {
  if (did.length <= 48) return did
  return `${did.slice(0, 22)}…${did.slice(-18)}`
}

export function LoginPage() {
  const { state, enroll, login, logout } = useLogin()
  const wasm = useWasm()
  const [view, setView] = useState<'signin' | 'create'>('signin')

  if (state.status === 'success') {
    return (
      <>
        <p className="tui-line">
          <span className="tui-line-dim">did</span>
          <span className="tui-line-value" title={state.session.did}>
            {truncateDid(state.session.did)}
          </span>
        </p>
        <button
          type="button"
          className="tui-primary tui-primary-outline"
          onClick={() => void logout()}
        >
          ■ disconnect
        </button>
        <SignerRequestPanel />
      </>
    )
  }

  if (state.status === 'restoring') {
    return (
      <p className="tui-msg" aria-live="polite">
        <span className="tui-blink">▮</span> checking session
      </p>
    )
  }

  if (wasm.status === 'failed') {
    return (
      <p role="alert" className="tui-msg tui-msg-error">
        wallet engine: {wasm.message}
      </p>
    )
  }

  const loading = state.status === 'loading'
  const wasmReady = wasm.status === 'ready'

  // Production login is passkey-only — the seed-enrollment view exists only in demo mode.
  if (seedEnrollEnabled && view === 'create') {
    return (
      <>
        <SeedInputForm
          loading={loading}
          wasmReady={wasmReady}
          hint="seal your seed under a new passkey (one time)"
          action="▶ create passkey from seed"
          busyLabel="creating passkey"
          onSubmit={(phrase) => void enroll(phrase)}
        />
        <LoginResult state={state} />
        <p className="tui-secondary">
          <button
            type="button"
            className="tui-link"
            disabled={loading}
            onClick={() => {
              setView('signin')
            }}
          >
            ← back to sign in
          </button>
        </p>
      </>
    )
  }

  return (
    <>
      <LoginForm loading={loading} wasmReady={wasmReady} onAuthenticate={() => void login()} />
      <LoginResult state={state} />
      {seedEnrollEnabled ? (
        <p className="tui-secondary">
          <button
            type="button"
            className="tui-link"
            disabled={loading}
            onClick={() => {
              setView('create')
            }}
          >
            Create a passkey
          </button>
        </p>
      ) : (
        <p className="tui-secondary tui-line-dim">No passkey yet? Set one up in the PEGIN app.</p>
      )}
    </>
  )
}
