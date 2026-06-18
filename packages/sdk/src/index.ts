/**
 * @pegin/sdk public surface — import only from here, never from internal paths.
 */

// entities
export type { PeginSession } from './entities/session/index.js'
export { selectIsExpired, selectDisplayName, selectDid } from './entities/session/index.js'
export type { ChiaDid } from './entities/did/index.js'
export { isChiaDid, launcherIdFromDid, didFromLauncherId } from './entities/did/index.js'

// features
export {
  PeginButton,
  loginWithPegin,
  loginWithPasskey,
  loadPeginSession,
  logoutPegin,
  PeginAuthClient,
} from './features/login/index.js'
export {
  PasskeySigner,
  enrollPasskey,
  localStorageVault,
  isPasskeyEnrolled,
} from './features/login/index.js'
export type {
  PeginButtonOptions,
  LoginOptions,
  LoginWithPeginOptions,
  LoginWithPasskeyOptions,
  PeginSigner,
  SignLoginRequest,
  SignedLogin,
  IdentityKey,
  PasskeySignerOptions,
  EnrollPasskeyOptions,
  WebAuthnApi,
  PasskeyVaultStore,
  VaultBlob,
  PeginNoncePayload,
  PeginServerSession,
  PeginResolvedOwner,
} from './features/login/index.js'
export { beginAuthentication, finishAuthentication } from './features/login/index.js'
export { beginRegistration, finishRegistration } from './features/register/index.js'
export type { RegisterOptions } from './features/register/index.js'
export {
  requestSignerSignMessage,
  startSignMessageRequest,
  pollSignRequest,
  wakePeginSigner,
} from './features/wallet/signer-request.js'
export type {
  SignRequestStart,
  SignRequestPoll,
  StartSignMessageParams,
} from './features/wallet/signer-request.js'

// shared utilities (opt-in — not part of the primary API)
export { decodeJwtPayload, isJwtExpired } from './shared/lib/jwt.js'
export { normalizeWebAuthnRpId } from './shared/lib/rpid.js'
export type { IsoTimestamp, Base64Url, HexString } from './shared/types/index.js'
