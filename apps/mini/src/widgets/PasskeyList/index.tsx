// Registered passkey backups — read-only list for locked screens.

import type { PasskeyBackupInfo } from '../../shared/ipc/index.js'

function shortId(credentialId: string): string {
  if (credentialId.length <= 12) return credentialId
  return `${credentialId.slice(0, 8)}…${credentialId.slice(-4)}`
}

interface PasskeyListProps {
  passkeys: PasskeyBackupInfo[]
  emptyMessage?: string
}

/** Displays passkey labels and credential ids without enrollment controls. */
export function PasskeyList({ passkeys, emptyMessage }: PasskeyListProps) {
  if (passkeys.length === 0) {
    return <p className="tui-msg">{emptyMessage ?? 'no passkeys registered yet'}</p>
  }

  return (
    <ul className="tui-backups">
      {passkeys.map((pk) => (
        <li key={pk.credentialId}>
          <span className="tui-line-value">{pk.label}</span>
          <span className="tui-line-dim"> · {shortId(pk.credentialId)}</span>
        </li>
      ))}
    </ul>
  )
}
