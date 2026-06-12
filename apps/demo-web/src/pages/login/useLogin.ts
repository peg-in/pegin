// Login flow: cookie restore → loginWithSeed (derive + lookup + mint in WASM) → cookie save.

import { useCallback, useRef, useState } from 'react'
import { clearSession, loadSession, saveSession } from '../../entities/session/index.js'
import { loadPeginWasm } from '../../shared/api/pegin-wasm.js'

export type LoginState =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; did: string; jwt: string }
  | { status: 'error'; message: string }

const JWT_TTL_SECONDS = 3600

const NO_DID_ON_CHAIN = 'no on-chain DID found for these keys'

function initialState(): LoginState {
  const session = loadSession()
  return session ? { status: 'success', ...session } : { status: 'idle' }
}

function loginErrorMessage(err: unknown): string {
  const msg = err instanceof Error ? err.message : String(err)
  if (msg === 'invalid mnemonic') return 'Invalid seed phrase'
  if (msg === NO_DID_ON_CHAIN) {
    return 'No active DID on testnet11 for this seed phrase — use a wallet that created a DID on testnet'
  }
  return 'DID lookup failed — check your connection and try again'
}

export function useLogin(): {
  state: LoginState
  login: (seedPhrase: string) => Promise<void>
  logout: () => void
} {
  const [state, setState] = useState<LoginState>(initialState)
  const busy = useRef(false)

  const login = useCallback(async (seedPhrase: string) => {
    if (busy.current) return
    const mnemonic = seedPhrase.trim()
    if (!mnemonic) {
      setState({ status: 'error', message: 'Seed phrase is required' })
      return
    }

    busy.current = true
    try {
      const wasm = await loadPeginWasm()
      setState({ status: 'loading' })

      try {
        const result = await wasm.loginWithSeed(mnemonic, null, JWT_TTL_SECONDS)
        saveSession(result.jwt)
        setState({ status: 'success', did: result.did, jwt: result.jwt })
      } catch (err) {
        setState({ status: 'error', message: loginErrorMessage(err) })
      }
    } finally {
      busy.current = false
    }
  }, [])

  const logout = useCallback(() => {
    clearSession()
    setState({ status: 'idle' })
  }, [])

  return { state, login, logout }
}
