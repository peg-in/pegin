// Vault summary — wallet name, passkeys, and navigation when a vault already exists.

import type { PasskeyBackupInfo, VaultStatus } from '../../shared/ipc/index.js'
import { PasskeyList } from '../../widgets/PasskeyList/index.js'

interface VaultSummaryProps {
  status: VaultStatus | null
  passkeys: PasskeyBackupInfo[]
}

/** Read-only snapshot of what is already on this device. */
export function VaultSummary({ status, passkeys }: VaultSummaryProps) {
  if (!status?.sealed) return null

  return (
    <section className="tui-connection-card">
      <span className="tui-field-label">on this device</span>
      <p className="tui-line">
        <span className="tui-line-dim">wallet</span>
        <span className="tui-line-value">{status.name ?? 'unnamed'}</span>
      </p>
      <p className="tui-field-label">passkeys</p>
      <PasskeyList
        passkeys={passkeys}
        emptyMessage="no passkeys yet — unlock and register one for web login"
      />
    </section>
  )
}
