import { initialAuthState } from '../../features/authenticate/index.js'

// Stub — composes PeginButton + status display. Full impl in feat-9.
export function LoginCard() {
  const state = initialAuthState

  return (
    <section style={{ border: '1px solid #ddd', borderRadius: 8, padding: '1.5rem' }}>
      <h2 style={{ marginTop: 0 }}>Login with PEGIN</h2>
      <p style={{ color: '#888' }}>
        Status: <strong>{state.status}</strong>
      </p>
      <button disabled style={{ opacity: 0.5 }}>
        Login with PEGIN (feat-9)
      </button>
    </section>
  )
}
