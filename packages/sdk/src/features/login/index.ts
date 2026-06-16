export { PeginButton } from './ui/PeginButton.js'
export type { PeginButtonOptions } from './ui/PeginButton.js'
export { beginAuthentication, finishAuthentication } from './model/index.js'
export type { LoginOptions } from './model/index.js'
export { loginWithPegin, loadPeginSession, logoutPegin } from './login.service.js'
export type { LoginWithPeginOptions } from './login.service.js'
export type {
  PeginSigner,
  SignLoginRequest,
  SignedLogin,
  IdentityKey,
} from './signers/pegin-signer.js'
export { PasskeySigner, enrollPasskey } from './signers/passkey-signer.js'
export type {
  PasskeySignerOptions,
  EnrollPasskeyOptions,
  WebAuthnApi,
} from './signers/passkey-signer.js'
export { localStorageVault } from './signers/passkey-vault.js'
export type { PasskeyVaultStore, VaultBlob } from './signers/passkey-vault.js'
export { PeginAuthClient } from '../../shared/api/pegin-auth-api.js'
export type {
  PeginNoncePayload,
  PeginServerSession,
  PeginResolvedOwner,
} from '../../shared/api/pegin-auth-api.js'
