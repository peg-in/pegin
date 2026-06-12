import { verifyChallengeSignature } from '../challenge/challenge.service.js'
import { launcherExists, launcherIdHexFromDid } from '../did/did.repository.js'
import { verifyToken } from '../jwt/jwt.service.js'
import {
  VerifyError,
  type VerifiedLogin,
  type VerifyLoginInput,
} from '../../shared/types/verify.types.js'

/** Verifies JWT, optional challenge signature, and optional coinset DID anchor. */
export async function verifyLogin(input: VerifyLoginInput): Promise<VerifiedLogin> {
  const now = input.now ?? Math.floor(Date.now() / 1000)
  const jwt = verifyToken(input.jwt, input.expectedAud, input.challengeNonce, now)

  if (input.challengeNonce !== undefined) {
    if (!input.challengeSigHex) {
      throw new VerifyError('challenge signature required')
    }
    try {
      await verifyChallengeSignature(jwt.didPkHex, input.challengeNonce, input.challengeSigHex)
    } catch {
      throw new VerifyError('challenge signature invalid')
    }
  }

  if (input.coinsetBaseUrl !== undefined) {
    const launcher = launcherIdHexFromDid(jwt.did)
    let anchored: boolean
    try {
      anchored = await launcherExists(input.coinsetBaseUrl, launcher)
    } catch (e) {
      throw new VerifyError(`coinset request failed: ${e instanceof Error ? e.message : String(e)}`)
    }
    if (!anchored) {
      throw new VerifyError('DID not anchored on chain')
    }
    // SECURITY (known gap — see feat-17): this proves the DID *exists*, not that
    // `cnf.did_pk` *owns* it. A caller who knows a victim's public DID can still mint a
    // token claiming it with their own key and pass the challenge (signed by their own key).
    // Closing this needs uncurrying the DID singleton's owner puzzle and matching it to
    // `synthetic(cnf.did_pk)` — tracked for the Rust pegin-verify path which has the tooling.
  }

  return {
    did: jwt.did,
    aud: jwt.aud,
    didPkHex: jwt.didPkHex,
    ...(jwt.nonce !== undefined ? { nonce: jwt.nonce } : {}),
  }
}
