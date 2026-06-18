// Recovery phrase display — reveal toggle, word grid, copy.

import { WordGrid } from '../WordGrid/index.js'

interface SeedPhrasePanelProps {
  words: string[]
  revealed: boolean
  busy: boolean
  copied: boolean
  locked?: boolean
  onRevealChange: (revealed: boolean) => void
  onCopy: () => void
}

/** Shows the BIP-39 phrase when unlocked; prompts to unlock when locked. */
export function SeedPhrasePanel({
  words,
  revealed,
  busy,
  copied,
  locked,
  onRevealChange,
  onCopy,
}: SeedPhrasePanelProps) {
  if (locked) {
    return (
      <section className="tui-section">
        <h2 className="tui-field-label">recovery phrase</h2>
        <p className="tui-msg">unlock this device to view your sealed seed phrase</p>
      </section>
    )
  }

  return (
    <section className="tui-section">
      <div className="tui-actions">
        <h2 className="tui-field-label">recovery phrase</h2>
        <button
          type="button"
          className="tui-reveal"
          onClick={() => {
            onRevealChange(!revealed)
          }}
          aria-pressed={revealed}
        >
          [{revealed ? 'hide' : 'show'}]
        </button>
      </div>

      {revealed && words.length > 0 ? (
        <>
          <WordGrid words={words} />
          <button type="button" className="tui-ghost" disabled={busy} onClick={onCopy}>
            {copied ? '✓ copied' : '⎘ copy seed phrase'}
          </button>
        </>
      ) : (
        <p className="tui-msg">{words.length > 0 ? `${words.length} words sealed` : 'loading phrase…'}</p>
      )}
    </section>
  )
}
