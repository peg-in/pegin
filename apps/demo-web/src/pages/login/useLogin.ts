// Login page hook — delegates to @pegin/sdk (passkey signer, server-verified session).

import { useCallback, useEffect, useRef, useState } from 'react'
import {
  enrollPasskey,
  isPasskeyEnrolled,
  loadPeginSession,
  loginWithPasskey,
  logoutPegin,
  normalizeWebAuthnRpId,
  type PeginSession,
} from '@pegin/sdk'
import { seedEnrollEnabled } from '../../shared/lib/index.js'

export type LoginState =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'restoring' }
  | { status: 'success'; session: PeginSession }
  | { status: 'error'; message: string }

function loginErrorMessage(err: unknown): string {
  const msg = err instanceof Error ? err.message : String(err)
  if (msg === 'invalid mnemonic') return 'Invalid seed phrase'
  if (msg.includes('cancelled')) return 'Passkey prompt cancelled — try again'
  if (msg.includes('WebAuthn is not available')) {
    return 'This browser does not support passkeys'
  }
  if (msg.includes('login verification failed') || msg.includes('audience mismatch')) {
    return 'Login could not be verified — try again'
  }
  if (msg.includes('PRF')) {
    return 'This authenticator has no PRF support — use 1Password or a PRF-capable passkey'
  }
  if (msg.includes('enroll')) {
    return seedEnrollEnabled
      ? 'No passkey on this device yet — create one below'
      : 'No passkey on this device yet — set one up in the PEGIN app'
  }
  if (msg.includes('passkey vault on this browser') || msg.includes('passkey blob')) {
    return 'Passkey not synced to auth relay — start demo-web, re-register in Signer (or tap sync), then try again'
  }
  if (msg.includes('passkey decrypt failed')) {
    return 'Wrong passkey for this wallet — use the one from PEGIN Signer on http://localhost'
  }
  if (msg.includes('no on-chain DID') || msg.includes('could not resolve')) {
    return 'No active DID on testnet11 for this wallet — create a DID on testnet first'
  }
  if (
    msg.includes('upstream') ||
    msg.includes('passkey blob fetch failed') ||
    msg.includes('Failed to fetch') ||
    msg.includes('NetworkError')
  ) {
    return 'Auth relay not reachable — run demo-web at http://localhost and try again'
  }
  if (msg.includes('failed to start login') || msg.includes('login expired')) {
    return 'Auth session expired — refresh the page and sign in again'
  }
  if (msg.length > 0 && msg !== '[object Object]') {
    return `Login failed — ${msg}`
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
  const [enrolled, setEnrolled] = useState(() => isPasskeyEnrolled())
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

  const rpId = () => normalizeWebAuthnRpId(window.location.hostname)

  // Modal sign-in: the browser / 1Password passkey picker opens, the chosen passkey's PRF
  // secret decrypts the stored seed, and the relay verifies the resolved DID. loginWithPasskey
  // builds + disposes the signer (zeroizing keys) for us.
  const login = useCallback(async () => {
    if (busy.current) return
    busy.current = true
    setState({ status: 'loading' })
    try {
      const session = await loginWithPasskey({ rpId: rpId() })
      setState({ status: 'success', session })
    } catch (err) {
      setState({ status: 'error', message: loginErrorMessage(err) })
    } finally {
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
