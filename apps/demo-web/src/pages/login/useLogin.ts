// Login page hook — delegates to @pegin/sdk (passkey signer, server-verified session).

import { useCallback, useEffect, useRef, useState } from 'react'
import {
  enrollPasskey,
  loadPeginSession,
  localStorageVault,
  loginWithPegin,
  logoutPegin,
  PasskeySigner,
  type PeginSession,
} from '@pegin/sdk'

/** True once a seed has been sealed under a passkey on this device. */
function hasEnrolledPasskey(): boolean {
  try {
    return localStorageVault().load() !== null
  } catch {
    return false
  }
}

export type LoginState =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'restoring' }
  | { status: 'success'; session: PeginSession }
  | { status: 'error'; message: string }

function loginErrorMessage(err: unknown): string {
  const msg = err instanceof Error ? err.message : String(err)
  if (msg === 'invalid mnemonic') return 'Invalid seed phrase'
  if (msg.includes('no on-chain DID')) {
    return 'No active DID on testnet11 for this passkey — register a DID on a testnet wallet first'
  }
  if (msg.includes('PRF')) {
    return 'This authenticator has no PRF support — use 1Password or a PRF-capable passkey'
  }
  if (msg.includes('enroll')) return 'No passkey on this device yet — create one below'
  if (msg.includes('cancelled')) return 'Passkey prompt cancelled — try again'
  if (msg.includes('WebAuthn is not available')) {
    return 'This browser does not support passkeys'
  }
  if (msg.includes('login verification failed') || msg.includes('audience mismatch')) {
    return 'Login could not be verified — try again'
  }
  if (msg.includes('upstream')) {
    return 'Auth relay could not reach testnet — try again shortly'
  }
  return 'Login failed — check your connection and try again'
}

export function useLogin(): {
  state: LoginState
  enrolled: boolean
  enroll: (seedPhrase: string) => Promise<void>
  login: () => Promise<void>
  logout: () => Promise<void>
} {
  const [state, setState] = useState<LoginState>({ status: 'restoring' })
  const [enrolled, setEnrolled] = useState(hasEnrolledPasskey)
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

  const rpId = () => window.location.hostname

  // Modal sign-in: the browser / 1Password passkey picker opens, the chosen passkey's PRF
  // secret decrypts the stored seed, and the relay verifies the resolved DID.
  const login = useCallback(async () => {
    if (busy.current) return
    busy.current = true
    setState({ status: 'loading' })
    const signer = new PasskeySigner({ rpId: rpId() })
    try {
      const session = await loginWithPegin({ signer })
      setState({ status: 'success', session })
    } catch (err) {
      setState({ status: 'error', message: loginErrorMessage(err) })
    } finally {
      await signer.dispose().catch(() => undefined)
      busy.current = false
    }
  }, [])

  // One-time: seal the seed under a new passkey, then log straight in. The seed phrase is
  // entered only here (enrollment), never on the recurring passkey login path.
  const enroll = useCallback(
    async (seedPhrase: string) => {
      if (busy.current) return
      const mnemonic = seedPhrase.trim()
      if (!mnemonic) {
        setState({ status: 'error', message: 'Seed phrase is required to enroll' })
        return
      }
      busy.current = true
      setState({ status: 'loading' })
      try {
        await enrollPasskey({ rpId: rpId(), userName: 'pegin', mnemonic })
        setEnrolled(true)
      } catch (err) {
        setState({ status: 'error', message: loginErrorMessage(err) })
        busy.current = false
        return
      }
      busy.current = false
      await login()
    },
    [login],
  )

  const logout = useCallback(async () => {
    await logoutPegin()
    setState({ status: 'idle' })
  }, [])

  return { state, enrolled, enroll, login, logout }
}
