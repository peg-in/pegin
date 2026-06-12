// Login page — signed-in session view, or the seed phrase form gated on WASM readiness.

import { LoginForm } from './LoginForm.js'
import { LoginResult } from './LoginResult.js'
import { useLogin } from './useLogin.js'
import { useWasm } from './useWasm.js'

export function LoginPage() {
  const { state, login, logout } = useLogin()
  const wasm = useWasm()

  if (state.status === 'success') {
    return (
      <section className="card">
        <h2>Signed in</h2>
        <p className="hint">Session persists across reloads until the JWT expires.</p>
        <LoginResult state={state} />
        <button type="button" className="btn btn-outline" onClick={logout}>
          Logout
        </button>
      </section>
    )
  }

  return (
    <section className="card">
      <h2>Login with PEGIN</h2>
      {wasm.status === 'failed' ? (
        <p role="alert" className="alert">
          Wallet engine failed to load: {wasm.message}
        </p>
      ) : (
        <>
          <LoginForm
            loading={state.status === 'loading'}
            wasmReady={wasm.status === 'ready'}
            onSubmit={(phrase) => void login(phrase)}
          />
          <LoginResult state={state} />
        </>
      )}
    </section>
  )
}
