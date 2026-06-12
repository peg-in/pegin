// Result area below the form: progress, error message, or DID + truncated JWT.

import type { LoginState } from './useLogin.js'

const JWT_PREVIEW_CHARS = 40

export function LoginResult({ state }: { state: LoginState }) {
  if (state.status === 'idle') return null

  if (state.status === 'loading') {
    return (
      <p className="progress" aria-live="polite">
        Logging in…
      </p>
    )
  }

  if (state.status === 'error') {
    return (
      <p role="alert" className="alert">
        {state.message}
      </p>
    )
  }

  return (
    <dl className="result" aria-live="polite">
      <div>
        <dt>DID</dt>
        <dd>
          <code>{state.did}</code>
        </dd>
      </div>
      <div>
        <dt>JWT (first {JWT_PREVIEW_CHARS} chars)</dt>
        <dd>
          <code>{state.jwt.slice(0, JWT_PREVIEW_CHARS)}…</code>
        </dd>
      </div>
    </dl>
  )
}
