// Dashboard — wallet home after create, import, or sign-in.

import type { SubmitEvent } from 'react'
import type { PasskeyBackupInfo, VaultStatus } from '../../shared/ipc/index.js'
import { PasskeyStatus } from '../../widgets/PasskeyStatus/index.js'
import { WordGrid } from '../../widgets/WordGrid/index.js'

interface DashboardPageProps {
  status: VaultStatus | null
  passkeys: PasskeyBackupInfo[]
  seedWords: string[]
  seedRevealed: boolean
  seedCopied: boolean
  passkeyLabel: string
  busy: boolean
  showSetupHint: boolean
  onPasskeyLabelChange: (value: string) => void
  onAddPasskey: (event: SubmitEvent<HTMLFormElement>) => void
  onResyncPasskey?: (credentialId: string) => void
  onSeedRevealChange: (revealed: boolean) => void
  onCopyMnemonic: () => void
  onLock: () => void
  onDismissSetupHint: () => void
}

/** Unlocked home — recovery keys and passkey registration for web login. */
export function DashboardPage({
  status,
  passkeys,
  seedWords,
  seedRevealed,
  seedCopied,
  passkeyLabel,
  busy,
  showSetupHint,
  onPasskeyLabelChange,
  onAddPasskey,
  onResyncPasskey,
  onSeedRevealChange,
  onCopyMnemonic,
  onLock,
  onDismissSetupHint,
}: DashboardPageProps) {
  return (
    <>
      <div className="tui-manage-header">
        <div>
          <h1 className="tui-page-title">dashboard</h1>
          <p className="tui-line">
            <span className="tui-line-dim">wallet</span>
            <span className="tui-line-value">{status?.name ?? 'unnamed'}</span>
          </p>
        </div>
        <button type="button" className="tui-reveal" disabled={busy} onClick={onLock}>
          [lock]
        </button>
      </div>

      {showSetupHint && (
        <section className="tui-sign-request" role="status">
          <p className="tui-field-label">wallet ready</p>
          <p className="tui-msg">
            Your recovery keys are below. Register a passkey next so websites can log you in and send
            sign requests to this app.
          </p>
          <button type="button" className="tui-ghost" onClick={onDismissSetupHint}>
            got it
          </button>
        </section>
      )}

      <section className="tui-dashboard-panel">
        <div className="tui-actions">
          <h2 className="tui-field-label">recovery keys</h2>
          <button
            type="button"
            className="tui-reveal"
            onClick={() => {
              onSeedRevealChange(!seedRevealed)
            }}
            aria-pressed={seedRevealed}
          >
            [{seedRevealed ? 'hide' : 'show'}]
          </button>
        </div>
        <p className="tui-msg">
          {seedWords.length > 0
            ? `${seedWords.length} words · device-sealed bip-39 phrase`
            : 'loading keys…'}
        </p>
        {seedRevealed && seedWords.length > 0 && (
          <>
            <WordGrid words={seedWords} />
            <button type="button" className="tui-ghost" disabled={busy} onClick={onCopyMnemonic}>
              {seedCopied ? '✓ copied' : '⎘ copy seed phrase'}
            </button>
          </>
        )}
      </section>

      <section className="tui-dashboard-panel tui-dashboard-panel-accent">
        <h2 className="tui-field-label">passkeys for web</h2>
        <p className="tui-msg">
          {passkeys.length > 0
            ? `${passkeys.length} registered — browsers use these for Login with PEGIN and sign requests.`
            : 'no passkeys yet — create one below to connect websites to this signer.'}
        </p>
        <PasskeyStatus
          passkeys={passkeys}
          passkeyLabel={passkeyLabel}
          busy={busy}
          onPasskeyLabelChange={onPasskeyLabelChange}
          onAddPasskey={onAddPasskey}
          onResyncPasskey={onResyncPasskey}
          compact
        />
      </section>
    </>
  )
}
