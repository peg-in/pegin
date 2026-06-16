// Create-passkey form: enter the seed once; it is sealed under a new passkey. The seed is
// the only input that matters here — login afterwards is passkey-only.

import { useRef, useState } from 'react'
import type { SubmitEvent } from 'react'

interface SeedInputFormProps {
  loading: boolean
  wasmReady: boolean
  hint: string
  action: string
  busyLabel: string
  onSubmit: (seedPhrase: string) => void
}

export function SeedInputForm({
  loading,
  wasmReady,
  hint,
  action,
  busyLabel,
  onSubmit,
}: SeedInputFormProps) {
  const [seedPhrase, setSeedPhrase] = useState('')
  const [revealed, setRevealed] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null)

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    const phrase = seedPhrase.trim()
    setSeedPhrase('')
    setRevealed(false)
    if (inputRef.current) inputRef.current.value = ''
    onSubmit(phrase)
  }

  const busy = loading || !wasmReady

  return (
    <form onSubmit={handleSubmit}>
      <p className="tui-line">
        <span className="tui-line-dim">seed</span>
        <span className="tui-line-value">{hint}</span>
      </p>
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
          autoComplete="off"
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
            {busyLabel}
          </>
        ) : wasmReady ? (
          action
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
