//! Seed-phrase login — derive keys, resolve identity (cache or public scan), mint a JWT.
//!
//! Secrets stay in WASM; only the minted JWT, its signature, and public keys are returned.

use crate::modules::did_lookup::helper::wallet_fingerprint;
use crate::modules::did_lookup::service::{login_subject, lookup_did_inner};
use crate::modules::jwt::service::mint_jwt_with_sk;
use crate::modules::keys::service::derive_wallet_keys_inner;
use crate::modules::signing::service::sign_challenge_with_sk;

/// Login material minted in-WASM. Secret keys never cross to JS.
#[derive(Debug)]
pub struct LoginSession {
    /// Canonical `did:chia` once resolved (cache or first-login scan), else empty.
    pub did: String,
    pub jwt: String,
    pub challenge_sig: Option<String>,
    /// Cache key for [`crate::modules::did_lookup`] — lets the SDK address the profile.
    pub wallet_fp: String,
    /// Address key the JWT is signed with (`cnf.did_pk` owner).
    pub owner_index: u32,
}

/// Derives keys, resolves identity, and mints a JWT signed by the owner key.
///
/// The JWT's `cnf.did_pk` carries the owner public key; the auth relay independently
/// re-verifies on-chain ownership (feat-17). Identity resolution reads only public chain
/// data — no secret ever leaves the browser.
///
/// * `scan_limit` — highest address index probed on a first-login DID scan
pub async fn login_with_seed_inner(
    mnemonic: &str,
    scan_limit: u32,
    ttl_seconds: u32,
    aud: &str,
    challenge_nonce: Option<&str>,
) -> Result<LoginSession, String> {
    let keys = derive_wallet_keys_inner(mnemonic)?;
    let identity = lookup_did_inner(&keys, scan_limit).await?;

    let owner_sk = keys.owner_secret_at(identity.owner_index);
    let subject = login_subject(&identity);
    let challenge_sig = challenge_nonce.map(|nonce| sign_challenge_with_sk(&owner_sk, nonce));
    let jwt = mint_jwt_with_sk(&owner_sk, &subject, aud, ttl_seconds, challenge_nonce)?;

    Ok(LoginSession {
        did: identity.did.unwrap_or_default(),
        jwt,
        challenge_sig,
        wallet_fp: wallet_fingerprint(&keys),
        owner_index: identity.owner_index,
    })
}

// Native-only: these drive the async login through `#[tokio::test]`; tokio is a
// native dev-dependency (it does not build for wasm32).
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::modules::jwt::service::verify_jwt_inner;
    use crate::test_util::deterministic_test_phrase;

    const AUD: &str = "https://app.example";

    // Native has no browser fetch, so the scan is a no-op: identity falls back to the
    // default owner key with no DID — exactly the first-login, pre-scan shape.
    #[tokio::test]
    async fn mints_verifiable_jwt_without_network() {
        let session = login_with_seed_inner(
            &deterministic_test_phrase(),
            10_000,
            3600,
            AUD,
            Some("nonce-1"),
        )
        .await
        .unwrap();

        assert!(verify_jwt_inner(&session.jwt, AUD, Some("nonce-1")).unwrap());
        assert!(session.challenge_sig.is_some());
        assert!(session.did.is_empty());
        assert_eq!(session.owner_index, 0);
        assert_eq!(session.wallet_fp.len(), 96);
    }

    #[tokio::test]
    async fn omits_challenge_sig_without_nonce() {
        let session = login_with_seed_inner(&deterministic_test_phrase(), 10_000, 3600, AUD, None)
            .await
            .unwrap();
        assert!(session.challenge_sig.is_none());
        assert!(verify_jwt_inner(&session.jwt, AUD, None).unwrap());
    }

    #[tokio::test]
    async fn rejects_invalid_mnemonic() {
        let err = login_with_seed_inner("not a valid mnemonic", 10_000, 3600, AUD, None)
            .await
            .unwrap_err();
        assert_eq!(err, "invalid mnemonic");
    }
}
