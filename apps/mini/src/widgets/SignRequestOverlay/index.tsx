// Pending web sign request — shown above any screen until approved or rejected.

import type { PendingSignRequest } from '../../shared/ipc/index.js'

interface SignRequestOverlayProps {
  request: PendingSignRequest
  canSign: boolean
  busy: boolean
  onApprove: () => void
  onReject: () => void
}

/** Relay sign-request card — user must unlock before approve is enabled. */
export function SignRequestOverlay({
  request,
  canSign,
  busy,
  onApprove,
  onReject,
}: SignRequestOverlayProps) {
  return (
    <section className="tui-sign-request" role="alert">
      <h2 className="tui-field-label">sign request from web app</h2>
      <p className="tui-line">
        <span className="tui-line-dim">site</span>
        <span className="tui-line-value">{request.origin}</span>
      </p>
      <p className="tui-line">
        <span className="tui-line-dim">action</span>
        <span className="tui-line-value">{request.summary}</span>
      </p>
      {request.message && <p className="tui-msg tui-line-value">{request.message}</p>}
      {!canSign && <p className="tui-warn">unlock your wallet to review and sign</p>}
      {canSign && (
        <div className="tui-actions">
          <button type="button" className="tui-primary" disabled={busy} onClick={onApprove}>
            ▶ approve & sign
          </button>
          <button type="button" className="tui-ghost" disabled={busy} onClick={onReject}>
            reject
          </button>
        </div>
      )}
    </section>
  )
}
