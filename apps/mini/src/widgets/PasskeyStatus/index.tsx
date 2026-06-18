// Passkey backup list and enrollment form — no password confirmation.

import type { SubmitEvent } from 'react'
import type { PasskeyBackupInfo } from '../../shared/ipc/index.js'

interface PasskeyStatusProps {
  passkeys: PasskeyBackupInfo[]
  passkeyLabel: string
  busy: boolean
  onPasskeyLabelChange: (value: string) => void
  onAddPasskey: (event: SubmitEvent<HTMLFormElement>) => void
  onResyncPasskey?: ((credentialId: string) => void) | undefined
}

/** Primary manage view — passkeys enable web login without keys in the browser. */
export function PasskeyStatus({
  passkeys,
  passkeyLabel,
  busy,
  onPasskeyLabelChange,
  onAddPasskey,
  onResyncPasskey,
  compact,
}: PasskeyStatusProps & { compact?: boolean }) {
  const Wrapper = compact ? 'div' : 'section'

  return (
    <Wrapper className={compact ? undefined : 'tui-section tui-section-first'}>
      {!compact && (
        <>
          <h2 className="tui-field-label">passkey backups</h2>
          <p className="tui-msg tui-msg-info">
            Register a passkey so websites can log you in and route sign requests here. No passwords
            — Face ID / Touch ID + relay session only.
          </p>
        </>
      )}

      {passkeys.length > 0 ? (
        <ul className="tui-backups">
          {passkeys.map((pk) => (
            <li key={pk.credentialId}>
              <span className="tui-line-value">{pk.label}</span>
              <span className="tui-line-dim">
                {' '}
                · {pk.credentialId.length > 12 ? `${pk.credentialId.slice(0, 8)}…` : pk.credentialId}
              </span>
              {onResyncPasskey && (
                <button
                  type="button"
                  className="tui-ghost"
                  disabled={busy}
                  onClick={() => {
                    onResyncPasskey(pk.credentialId)
                  }}
                >
                  sync for web login
                </button>
              )}
            </li>
          ))}
        </ul>
      ) : (
        <p className="tui-msg">no passkeys yet — add one to connect websites to this signer</p>
      )}

      <form onSubmit={onAddPasskey}>
        <div className="tui-prompt-row">
          <label className="tui-prompt" htmlFor="pk-label">
            label&gt;
          </label>
          <input
            id="pk-label"
            className="tui-input"
            type="text"
            value={passkeyLabel}
            onChange={(e) => {
              onPasskeyLabelChange(e.target.value)
            }}
            placeholder="e.g. YubiKey, 1Password, iPhone"
            disabled={busy}
          />
        </div>
        <button
          type="submit"
          className="tui-primary"
          disabled={busy || !passkeyLabel.trim()}
        >
          + register passkey for web
        </button>
      </form>
    </Wrapper>
  )
}