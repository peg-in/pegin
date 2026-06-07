// Login flow model — beginAuthentication + finishAuthentication.
// Full implementation in feat-9 (pegin-wasm browser bundle).

import type { PeginSession } from "../../../entities/session/index.js";

export interface LoginOptions {
  rpId: string;
  apiBaseUrl: string;
}

export async function beginAuthentication(_options: LoginOptions): Promise<void> {
  throw new Error("beginAuthentication — not yet implemented (feat-9)");
}

export async function finishAuthentication(
  _options: LoginOptions,
  _assertionResponse: unknown,
): Promise<PeginSession> {
  throw new Error("finishAuthentication — not yet implemented (feat-9)");
}
