// Sign in — unlock device-sealed wallet; setup tabs for create | import.

import type { PasskeyBackupInfo, VaultStatus } from '../../shared/ipc/index.js'
import { type SetupMode } from '../../shared/ui/SetupTabs.js'
import { PasskeyList } from '../../widgets/PasskeyList/index.js'

interface UnlockPageProps {
  status: VaultStatus | null
  passkeys: PasskeyBackupInfo[]
  busy: boolean
  onUnlock: () => void
  onSetup: (mode: SetupMode) => void
}

/** Sign in to an existing wallet, or switch to create / import tabs. */
export function UnlockPage({ status, passkeys, busy, onUnlock, onSetup }: UnlockPageProps) {
  return (
    <>
      <h1 className="tui-page-title">sign in</h1>
      <p className="tui-line">
        <span className="tui-line-dim">wallet</span>
        <span className="tui-line-value">{status?.name ?? 'unnamed'}</span>
      </p>

      <section className="tui-connection-card">
        <span className="tui-field-label">registered passkeys</span>
        <PasskeyList
          passkeys={passkeys}
          emptyMessage="no passkeys yet — unlock and register one on the dashboard"
        />
      </section>

      <button
        type="button"
        className="tui-primary"
        disabled={busy || (!!status?.sealed && !status.hasDeviceUnlock)}
        onClick={onUnlock}
      >
        {busy ? (
          <>
            <span className="tui-spinner" aria-hidden="true" />
            unlocking
          </>
        ) : (
          '▶ unlock with this device'
        )}
      </button>

      {!status?.hasDeviceUnlock && status?.sealed && (
        <p className="tui-warn">
          no device key found — re-import the same seed below to restore access
          {status.passkeyCount > 0 ? ' (your passkeys will be kept)' : ''}
        </p>
      )}

      {status && status.passkeyCount > passkeys.length && (
        <p className="tui-msg">
          {status.passkeyCount} passkey{status.passkeyCount === 1 ? '' : 's'} on file — unlock to
          manage
        </p>
      )}

      <section className="tui-section">
        <p className="tui-field-label">different wallet?</p>
        <div className="tui-tabs" role="group" aria-label="setup wallet">
          <button
            type="button"
            className="tui-tab"
            onClick={() => {
              onSetup('create')
            }}
          >
            create
          </button>
          <button
            type="button"
            className="tui-tab"
            onClick={() => {
              onSetup('import')
            }}
          >
            import
          </button>
        </div>
        <p className="tui-msg">switch wallet — replaces what is sealed on this device</p>
      </section>
    </>
  )
}
