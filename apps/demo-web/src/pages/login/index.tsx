// Login page — one primary action, terminal-minimal states.

import { LoginForm } from './LoginForm.js'
import { LoginResult } from './LoginResult.js'
import { useLogin } from './useLogin.js'
import { useWasm } from './useWasm.js'

function truncateDid(did: string): string {
  if (did.length <= 48) return did
  return `${did.slice(0, 22)}…${did.slice(-18)}`
}

export function LoginPage() {
  const { state, login, logout } = useLogin()
  const wasm = useWasm()

  if (state.status === 'success') {
    return (
      <>
        <p className="tui-line">
          <span className="tui-line-dim">did</span>
          <span className="tui-line-value" title={state.session.did}>
            {truncateDid(state.session.did)}
          </span>
        </p>
        <button type="button" className="tui-primary tui-primary-outline" onClick={() => void logout()}>
          ■ disconnect
        </button>
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

  return (
    <>
      <LoginForm
        loading={state.status === 'loading'}
        wasmReady={wasm.status === 'ready'}
        onSubmit={(phrase) => void login(phrase)}
      />
      <LoginResult state={state} />
    </>
  )
}
