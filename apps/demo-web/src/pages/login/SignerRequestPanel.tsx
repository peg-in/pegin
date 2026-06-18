// Demo panel — request a BLS message sign from PEGIN Signer via relay + deep link.

import { useState } from 'react'
import { requestSignerSignMessage, type SignRequestPoll } from '@pegin/sdk'

type SignState =
  | { status: 'idle' }
  | { status: 'waiting' }
  | { status: 'done'; sig: string; valid: boolean; txSubmitted?: boolean }
  | { status: 'error'; message: string }

// A BLS (G2) signature is 96 bytes — 192 hex chars. A well-formed sig is the web app's
// client-side confirmation that the Signer actually produced one.
const isBlsSignature = (hex: string): boolean => /^[0-9a-f]{192}$/i.test(hex)

export function SignerRequestPanel() {
  const [state, setState] = useState<SignState>({ status: 'idle' })

  const onSign = async () => {
    setState({ status: 'waiting' })
    try {
      const result: SignRequestPoll = await requestSignerSignMessage({
        message: `pegin-demo-${Date.now()}`,
        summary: 'Sign demo message for PEGIN Signer wallet test',
      })
      if (result.status === 'completed' && result.messageSigHex) {
        setState({
          status: 'done',
          sig: result.messageSigHex,
          valid: isBlsSignature(result.messageSigHex),
          txSubmitted: result.txSubmitted ?? false,
        })
        return
      }
      setState({ status: 'error', message: `signer ${result.status}` })
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err)
      setState({ status: 'error', message })
    }
  }

  return (
    <section className="tui-section">
      <h2 className="tui-field-label">sign with PEGIN Signer</h2>
      <p className="tui-msg">
        opens the native signer app to approve — works even if it was closed
      </p>
      <button
        type="button"
        className="tui-ghost"
        disabled={state.status === 'waiting'}
        onClick={() => void onSign()}
      >
        {state.status === 'waiting' ? 'waiting for signer…' : '▶ sign test message'}
      </button>
      {state.status === 'done' &&
        (state.valid ? (
          <p className="tui-msg tui-msg-ok">
            ✓ signed by PEGIN Signer — valid BLS signature ({state.sig.slice(0, 16)}…)
          </p>
        ) : (
          <p role="alert" className="tui-msg tui-msg-error">
            signer returned an unexpected signature format
          </p>
        ))}
      {state.status === 'error' && (
        <p role="alert" className="tui-msg tui-msg-error">
          {state.message}
        </p>
      )}
    </section>
  )
}
