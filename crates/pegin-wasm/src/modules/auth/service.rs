//! Login signing — derive the owner key at the relay-resolved index, mint a JWT, sign the
//! challenge. Secrets stay in WASM; only the JWT, its signature, and public keys leave.

use crate::modules::jwt::service::mint_jwt_with_sk;
use crate::modules::keys::WalletKeys;
use crate::modules::signing::service::sign_challenge_with_sk;

/// Signed login material minted in-WASM. Secret keys never cross to JS.
#[derive(Debug)]
pub struct SignedLogin {
    pub jwt: String,
    pub challenge_sig: Option<String>,
}

/// Mints a JWT signed by the owner key at `owner_index` and signs the login challenge.
///
/// The relay already resolved `(did, owner_index)` from the watch-only account key, so the
/// JWT's `iss`/`sub` is the canonical DID and `cnf.did_pk` is the owner key the relay
/// re-verifies on-chain (feat-17). No chain I/O happens here.
///
/// * `owner_index` — DID-owning address index returned by `/resolve`
/// * `challenge_nonce` — when set, the owner key signs it and the sig is returned
pub fn sign_login_inner(
    keys: &WalletKeys,
    did: &str,
    owner_index: u32,
    aud: &str,
    ttl_seconds: u32,
    challenge_nonce: Option<&str>,
) -> Result<SignedLogin, String> {
    let owner_sk = keys.owner_secret_at(owner_index);
    let challenge_sig = challenge_nonce.map(|nonce| sign_challenge_with_sk(&owner_sk, nonce));
    let jwt = mint_jwt_with_sk(&owner_sk, did, aud, ttl_seconds, challenge_nonce)?;
    Ok(SignedLogin { jwt, challenge_sig })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::jwt::service::verify_jwt_inner;
    use crate::modules::keys::service::derive_wallet_keys_inner;
    use crate::test_util::deterministic_test_phrase;

    const AUD: &str = "https://app.example";
    const DID: &str = "did:chia:deadbeef01020304050607080900aabbccddeeff01020304050607080900aabb";

    #[test]
    fn mints_verifiable_jwt_for_resolved_did() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let signed = sign_login_inner(&keys, DID, 0, AUD, 3600, Some("nonce-1")).unwrap();

        assert!(verify_jwt_inner(&signed.jwt, AUD, Some("nonce-1")).unwrap());
        assert!(signed.challenge_sig.is_some());
    }

    #[test]
    fn omits_challenge_sig_without_nonce() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let signed = sign_login_inner(&keys, DID, 0, AUD, 3600, None).unwrap();

        assert!(signed.challenge_sig.is_none());
        assert!(verify_jwt_inner(&signed.jwt, AUD, None).unwrap());
    }

    #[test]
    fn different_owner_indexes_sign_with_different_keys() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let a = sign_login_inner(&keys, DID, 0, AUD, 3600, Some("n")).unwrap();
        let b = sign_login_inner(&keys, DID, 7, AUD, 3600, Some("n")).unwrap();
        assert_ne!(a.challenge_sig, b.challenge_sig);
    }
}
