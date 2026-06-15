// Login page hook — delegates to @pegin/sdk (server-verified session, dynamic aud).

import { useCallback, useEffect, useRef, useState } from 'react'
import { loadPeginSession, loginWithPegin, logoutPegin, type PeginSession } from '@pegin/sdk'

export type LoginState =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'restoring' }
  | { status: 'success'; session: PeginSession }
  | { status: 'error'; message: string }

function loginErrorMessage(err: unknown): string {
  const msg = err instanceof Error ? err.message : String(err)
  if (msg === 'invalid mnemonic') return 'Invalid seed phrase'
  // Matches both relay messages: "no on-chain DID found…" and "…for this owner key".
  if (msg.includes('no on-chain DID')) {
    return 'No active DID on testnet11 for this seed phrase — use a wallet that created a DID on testnet'
  }
  if (msg.includes('login verification failed') || msg.includes('audience mismatch')) {
    return 'Login could not be verified — try again'
  }
  if (msg.includes('upstream verification unavailable')) {
    return 'Auth relay could not reach testnet — try again shortly'
  }
  return 'Login failed — check your connection and try again'
}

export function useLogin(): {
  state: LoginState
  login: (seedPhrase: string) => Promise<void>
  logout: () => Promise<void>
} {
  const [state, setState] = useState<LoginState>({ status: 'restoring' })
  const busy = useRef(false)

  useEffect(() => {
    let cancelled = false
    loadPeginSession()
      .then((session) => {
        if (cancelled) return
        setState(session ? { status: 'success', session } : { status: 'idle' })
      })
      .catch(() => {
        if (!cancelled) setState({ status: 'idle' })
      })
    return () => {
      cancelled = true
    }
  }, [])

  const login = useCallback(async (seedPhrase: string) => {
    if (busy.current) return
    const mnemonic = seedPhrase.trim()
    if (!mnemonic) {
      setState({ status: 'error', message: 'Seed phrase is required' })
      return
    }

    busy.current = true
    setState({ status: 'loading' })
    try {
      const session = await loginWithPegin(mnemonic)
      setState({ status: 'success', session })
    } catch (err) {
      setState({ status: 'error', message: loginErrorMessage(err) })
    } finally {
      busy.current = false
    }
  }, [])

  const logout = useCallback(async () => {
    await logoutPegin()
    setState({ status: 'idle' })
  }, [])

  return { state, login, logout }
}
