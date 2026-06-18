// Setup — create | import tabs, then seal passwordlessly on this device.

import type { SubmitEvent } from 'react'
import { SetupTabs, type SetupMode } from '../../shared/ui/SetupTabs.js'
import { WordGrid } from '../../widgets/WordGrid/index.js'

type WordCount = 12 | 24

interface SetupPageProps {
  mode: SetupMode
  name: string
  wordCount: WordCount
  created: string
  seedInput: string
  revealed: boolean
  busy: boolean
  copied: boolean
  replacingVault: boolean
  showBack: boolean
  onModeChange: (mode: SetupMode) => void
  onBack: () => void
  onNameChange: (value: string) => void
  onWordCountChange: (count: WordCount) => void
  onGenerate: () => void
  onSeedChange: (value: string) => void
  onRevealedChange: (revealed: boolean) => void
  onCopyPhrase: () => void
  onSealCreate: (event: SubmitEvent<HTMLFormElement>) => void
  onSealImport: (event: SubmitEvent<HTMLFormElement>) => void
}

/** Tabbed wallet onboarding — create a phrase or import an existing one. */
export function SetupPage({
  mode,
  name,
  wordCount,
  created,
  seedInput,
  revealed,
  busy,
  copied,
  replacingVault,
  showBack,
  onModeChange,
  onBack,
  onNameChange,
  onWordCountChange,
  onGenerate,
  onSeedChange,
  onRevealedChange,
  onCopyPhrase,
  onSealCreate,
  onSealImport,
}: SetupPageProps) {
  const importPhrase = seedInput.trim().toLowerCase().replace(/\s+/g, ' ')
  const createWords = created ? created.split(' ') : []
  const importWords = importPhrase ? importPhrase.split(' ') : []
  const canSealCreate = name.trim().length > 0 && created.length > 0 && !busy
  const canSealImport = name.trim().length > 0 && importPhrase.length > 0 && !busy

  const switchMode = (next: SetupMode) => {
    onModeChange(next)
  }

  return (
    <>
      {showBack && (
        <button type="button" className="tui-back" onClick={onBack}>
          ← back
        </button>
      )}

      <SetupTabs mode={mode} onModeChange={switchMode} />

      {replacingVault && (
        <p className="tui-warn">a wallet is already sealed here — continuing replaces it</p>
      )}

      <form onSubmit={mode === 'create' ? onSealCreate : onSealImport}>
        <div className="tui-prompt-row">
          <label className="tui-prompt" htmlFor="wallet-name">
            name&gt;
          </label>
          <input
            id="wallet-name"
            className="tui-input"
            type="text"
            value={name}
            onChange={(e) => {
              onNameChange(e.target.value)
            }}
            placeholder="e.g. pegin main wallet"
            autoComplete="username"
            disabled={busy}
          />
        </div>

        {mode === 'create' ? (
          <>
            <div className="tui-seg">
              <span>length</span>
              {([12, 24] as const).map((n) => (
                <button
                  key={n}
                  type="button"
                  className="tui-seg-btn"
                  aria-pressed={wordCount === n}
                  onClick={() => {
                    onWordCountChange(n)
                  }}
                >
                  {n} words
                </button>
              ))}
            </div>
            <button type="button" className="tui-ghost" disabled={busy} onClick={onGenerate}>
              {created ? '↻ regenerate phrase' : '✦ generate phrase'}
            </button>
            {created && (
              <p className="tui-warn">write these words down — they are your only backup</p>
            )}
            {createWords.length > 0 && (
              <>
                <WordGrid words={createWords} />
                <button type="button" className="tui-ghost" disabled={busy} onClick={onCopyPhrase}>
                  {copied ? '✓ copied' : '⎘ copy phrase to clipboard'}
                </button>
              </>
            )}
          </>
        ) : (
          <>
            <div className="tui-field">
              <label className="tui-field-label" htmlFor="seed">
                seed&gt;{' '}
                <span className="tui-line-dim">type · paste · fill from password manager</span>
              </label>
              <div className="tui-prompt-row">
                <input
                  id="seed"
                  className="tui-input"
                  type={revealed ? 'text' : 'password'}
                  value={seedInput}
                  onChange={(e) => {
                    onSeedChange(e.target.value)
                  }}
                  placeholder="twelve or twenty-four words"
                  autoComplete="off"
                  autoCapitalize="off"
                  autoCorrect="off"
                  spellCheck={false}
                  disabled={busy}
                />
                <button
                  type="button"
                  className="tui-reveal"
                  onClick={() => {
                    onRevealedChange(!revealed)
                  }}
                  aria-pressed={revealed}
                >
                  [{revealed ? 'hide' : 'show'}]
                </button>
              </div>
            </div>
            {importWords.length > 0 && (
              <>
                <WordGrid words={importWords} />
                <button type="button" className="tui-ghost" disabled={busy} onClick={onCopyPhrase}>
                  {copied ? '✓ copied' : '⎘ copy phrase to clipboard'}
                </button>
              </>
            )}
          </>
        )}

        <p className="tui-msg tui-msg-info">no password — sealed with this device only</p>

        <button
          type="submit"
          className="tui-primary"
          disabled={mode === 'create' ? !canSealCreate : !canSealImport}
        >
          {busy ? (
            <>
              <span className="tui-spinner" aria-hidden="true" />
              sealing
            </>
          ) : mode === 'create' ? (
            '▶ seal & open dashboard'
          ) : (
            '▶ import & open dashboard'
          )}
        </button>
      </form>
    </>
  )
}
