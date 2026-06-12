// Masked seed prompt + single authenticate button.

import { useRef, useState } from 'react'
import type { SubmitEvent } from 'react'

interface LoginFormProps {
  loading: boolean
  wasmReady: boolean
  onSubmit: (seedPhrase: string) => void
}

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
    wipeSeedInput(inputRef.current, () => {
      setSeedPhrase('')
      setRevealed(false)
    })
    onSubmit(phrase)
  }

  const busy = loading || !wasmReady

  return (
    <form onSubmit={handleSubmit}>
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
      <div className="tui-prompt-row">
        <label className="tui-prompt" htmlFor="seed-phrase">
          seed&gt;
        </label>
        <input
          ref={inputRef}
          id="seed-phrase"
          className="tui-input"
          type={revealed ? 'text' : 'password'}
          name="password"
          value={seedPhrase}
          onChange={(event) => {
            setSeedPhrase(event.target.value)
          }}
          placeholder="········"
          autoComplete="current-password"
          autoCapitalize="off"
          autoCorrect="off"
          spellCheck={false}
          disabled={loading}
          aria-label="Seed phrase"
        />
        <button
          type="button"
          className="tui-reveal"
          onClick={() => {
            setRevealed(!revealed)
          }}
          aria-pressed={revealed}
        >
          [{revealed ? 'hide' : 'show'}]
        </button>
      </div>
      <button type="submit" className="tui-primary" disabled={busy}>
        {loading ? (
          <>
            <span className="tui-spinner" aria-hidden="true" />
            authenticating
          </>
        ) : wasmReady ? (
          '▶ authenticate'
        ) : (
          <>
            <span className="tui-spinner" aria-hidden="true" />
            loading wasm
          </>
        )}
      </button>
    </form>
  )
}
