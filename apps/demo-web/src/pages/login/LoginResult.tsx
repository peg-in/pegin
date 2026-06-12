// Inline status below the form — errors only; success is handled by the page shell.

import type { LoginState } from './useLogin.js'

export function LoginResult({ state }: { state: LoginState }) {
  if (state.status === 'error') {
    return (
      <p role="alert" className="tui-msg tui-msg-error">
        {state.message}
      </p>
    )
  }

  return null
}
