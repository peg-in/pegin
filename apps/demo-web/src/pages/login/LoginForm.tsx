// Masked seed phrase input (password-manager fillable) + the one Login button.

import { useRef, useState } from 'react'
import type { SubmitEvent } from 'react'

interface LoginFormProps {
  loading: boolean
  wasmReady: boolean
  onSubmit: (seedPhrase: string) => void
}

/** Clears the controlled input and any lingering DOM value after hand-off to WASM. */
function wipeSeedInput(input: HTMLInputElement | null, clearState: () => void): void {
  clearState()
  if (input) input.value = ''
}

export function LoginForm({ loading, wasmReady, onSubmit }: LoginFormProps) {
  const [seedPhrase, setSeedPhrase] = useState('')
  const [revealed, setRevealed] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null)

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    const phrase = seedPhrase.trim()
    // Drop the phrase from React/DOM before async login — only the call argument retains
    // it until loginWithSeed runs inside WASM; nothing is written to storage.
    wipeSeedInput(inputRef.current, () => {
      setSeedPhrase('')
      setRevealed(false)
    })
    onSubmit(phrase)
  }

  return (
    <form onSubmit={handleSubmit}>
      {/* Invisible username anchors password managers (1Password & co.) to this login form. */}
      <input
        className="visually-hidden"
        type="text"
        name="username"
        autoComplete="username"
        value="pegin-did"
        readOnly
        tabIndex={-1}
        aria-hidden="true"
      />
      <div className="field-row">
        <label className="field-label" htmlFor="seed-phrase">
          Seed phrase
        </label>
        <button
          type="button"
          className="reveal-toggle"
          onClick={() => {
            setRevealed(!revealed)
          }}
          aria-pressed={revealed}
        >
          {revealed ? 'Hide' : 'Show'}
        </button>
      </div>
      <input
        ref={inputRef}
        id="seed-phrase"
        className="input"
        type={revealed ? 'text' : 'password'}
        name="password"
        value={seedPhrase}
        onChange={(event) => {
          setSeedPhrase(event.target.value)
        }}
        placeholder="24 words separated by spaces"
        autoComplete="current-password"
        autoCapitalize="off"
        autoCorrect="off"
        spellCheck={false}
        disabled={loading}
      />
      <p className="hint">
        Fills from your password manager. Testnet wallet only — never use a phrase that holds real
        funds.
      </p>
      <button type="submit" className="btn" disabled={loading || !wasmReady}>
        {loading ? (
          <>
            <span className="spinner" aria-hidden="true" /> Logging in…
          </>
        ) : wasmReady ? (
          'Login with PEGIN'
        ) : (
          <>
            <span className="spinner" aria-hidden="true" /> Loading wallet engine…
          </>
        )}
      </button>
    </form>
  )
}
