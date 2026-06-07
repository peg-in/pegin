// Registration flow model — beginRegistration + finishRegistration.
// Full implementation in feat-9.

import type { ChiaDid } from '../../../entities/did/index.js'

export interface RegisterOptions {
  rpId: string
  apiBaseUrl: string
  username: string
}

export function beginRegistration(_options: RegisterOptions): Promise<void> {
  return Promise.reject(new Error('beginRegistration — not yet implemented (feat-9)'))
}

export function finishRegistration(
  _options: RegisterOptions,
  _attestationResponse: unknown,
): Promise<ChiaDid> {
  return Promise.reject(new Error('finishRegistration — not yet implemented (feat-9)'))
}
