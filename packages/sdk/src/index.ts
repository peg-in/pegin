// @pegin/sdk public surface — import only from here, never from internal paths.

// entities
export type { PeginSession } from './entities/session/index.js'
export { selectIsExpired, selectDisplayName, selectDid } from './entities/session/index.js'
export type { ChiaDid } from './entities/did/index.js'
export { isChiaDid, launcherIdFromDid, didFromLauncherId } from './entities/did/index.js'

// features
export { PeginButton } from './features/login/index.js'
export type { PeginButtonOptions, LoginOptions } from './features/login/index.js'
export { beginAuthentication, finishAuthentication } from './features/login/index.js'
export { beginRegistration, finishRegistration } from './features/register/index.js'
export type { RegisterOptions } from './features/register/index.js'

// shared utilities (opt-in — not part of the primary API)
export { decodeJwtPayload, isJwtExpired } from './shared/lib/jwt.js'
export type { IsoTimestamp, Base64Url, HexString } from './shared/types/index.js'
