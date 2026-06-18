/**
 * Cross-app sign requests — web posts to relay, wakes PEGIN Signer via deep link, polls result.
 *
 * Future vault keys will use the same channel with `kind: "signSpendBundle"` and multisig payloads.
 */

export interface SignRequestStart {
  requestId: string
  deepLink: string
  relayUrl: string
}

export interface SignRequestPoll {
  status: 'pending' | 'completed' | 'rejected' | 'expired'
  txSubmitted?: boolean
  messageSigHex?: string
  signedBundleB64?: string
}

export interface StartSignMessageParams {
  message: string
  summary: string
  returnUrl?: string
}

const DEFAULT_RELAY = '/api/pegin'

/** Creates a sign request on the relay and returns the deep link to open PEGIN Signer. */
export async function startSignMessageRequest(
  params: StartSignMessageParams,
  relayBase = DEFAULT_RELAY,
): Promise<SignRequestStart> {
  // returnUrl is only for flows that can't poll (the Signer redirects the browser back to it).
  // When the page polls — as here — sending it just spawns a redundant tab, so it stays opt-in.
  const body: Record<string, unknown> = {
    kind: 'signMessage',
    summary: params.summary,
    message: params.message,
  }
  if (params.returnUrl) {
    body.returnUrl = params.returnUrl
  }
  const resp = await fetch(`${relayBase}/request/start`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(body),
  })
  if (!resp.ok) {
    const err = (await resp.json().catch(() => ({}))) as { error?: string }
    throw new Error(err.error ?? `sign request failed (${resp.status})`)
  }
  return resp.json() as Promise<SignRequestStart>
}

/** Opens PEGIN Signer — uses deep link; OS prompts if the app was closed. */
export function wakePeginSigner(deepLink: string): void {
  window.location.href = deepLink
}

/** Polls relay until the Signer completes, rejects, or the request expires. */
export async function pollSignRequest(
  requestId: string,
  relayBase = DEFAULT_RELAY,
  timeoutMs = 120_000,
): Promise<SignRequestPoll> {
  const started = Date.now()
  while (Date.now() - started < timeoutMs) {
    const resp = await fetch(`${relayBase}/request/poll?requestId=${encodeURIComponent(requestId)}`, {
      credentials: 'include',
    })
    if (!resp.ok) {
      throw new Error(`poll failed (${resp.status})`)
    }
    const body = (await resp.json()) as SignRequestPoll
    if (body.status !== 'pending') {
      return body
    }
    await new Promise((resolve) => {
      setTimeout(resolve, 800)
    })
  }
  return { status: 'expired' }
}

/** Full flow: create request → wake Signer → poll until done. */
export async function requestSignerSignMessage(
  params: StartSignMessageParams,
): Promise<SignRequestPoll> {
  const start = await startSignMessageRequest(params)
  wakePeginSigner(start.deepLink)
  return pollSignRequest(start.requestId)
}
