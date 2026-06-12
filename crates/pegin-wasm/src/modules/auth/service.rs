//! Seed-phrase login — derive, on-chain DID lookup, and JWT mint entirely in WASM.

use crate::modules::keys::service::derive_wallet_keys_inner;

/// Returned to JS when keys derive but coinset finds no unspent DID coin.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub const NO_DID_ON_CHAIN: &str = "no on-chain DID found for these keys";

/// Derives keys, resolves the on-chain DID, mints a JWT — secret keys never cross to JS.
#[cfg_attr(not(target_arch = "wasm32"), allow(clippy::unused_async))]
pub async fn login_with_seed_inner(
    mnemonic: &str,
    peer_url: Option<&str>,
    ttl_seconds: u32,
    aud: &str,
    challenge_nonce: Option<&str>,
) -> Result<(String, String, Option<String>), String> {
    let keys = derive_wallet_keys_inner(mnemonic)?;

    #[cfg(target_arch = "wasm32")]
    {
        use crate::modules::did::service::get_did_for_keys_inner;
        use crate::modules::jwt::service::mint_jwt_inner;
        use crate::modules::signing::service::sign_challenge_inner;

        let did = get_did_for_keys_inner(&keys, peer_url)
            .await?
            .ok_or_else(|| NO_DID_ON_CHAIN.to_owned())?;
        let challenge_sig = challenge_nonce.map(|nonce| sign_challenge_inner(&keys, nonce));
        let jwt = mint_jwt_inner(&keys, &did, aud, ttl_seconds, challenge_nonce)?;
        Ok((did, jwt, challenge_sig))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (keys, peer_url, ttl_seconds, aud, challenge_nonce);
        Err("loginWithSeed requires browser WASM".to_owned())
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    use crate::test_util::deterministic_test_phrase;

    #[tokio::test]
    async fn native_stub_rejects_login() {
        let err = login_with_seed_inner(&deterministic_test_phrase(), None, 3600, "https://app", None)
            .await
            .unwrap_err();
        assert!(err.contains("browser WASM"));
    }

    #[tokio::test]
    async fn rejects_invalid_mnemonic_before_network() {
        let err = login_with_seed_inner("not a valid mnemonic", None, 3600, "https://app", None)
            .await
            .unwrap_err();
        assert_eq!(err, "invalid mnemonic");
    }
}
