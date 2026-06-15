//! DID string forms + local profile cache keying.

use crate::modules::keys::WalletKeys;

/// Stable, non-secret cache key for a wallet's local profile.
///
/// The DID public key already crosses into JS (`didPkHex`), so keying the local
/// profile by it leaks nothing new and stays identical across sessions.
pub fn wallet_fingerprint(keys: &WalletKeys) -> String {
    keys.did_pk_hex()
}

// DID encoding + the launcher check are only used by the browser scan (and its tests).
#[cfg(any(target_arch = "wasm32", test))]
pub use did_codec::{encode_did, is_singleton_launcher};

#[cfg(any(target_arch = "wasm32", test))]
mod did_codec {
    use bech32::{Bech32m, Hrp};

    /// Human-readable part of a Chia DID per chia-blockchain's bech32m encoding.
    const DID_HRP: &str = "did:chia:";

    /// Standard Chia singleton launcher puzzle hash — every DID launcher coin uses it.
    const SINGLETON_LAUNCHER_PUZZLE_HASH: &str =
        "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";

    /// Encodes a launcher ID (64-char hex) as the canonical bech32m `did:chia:1…` string.
    pub fn encode_did(launcher_id_hex: &str) -> Result<String, String> {
        let bytes =
            hex::decode(launcher_id_hex).map_err(|e| format!("invalid launcher hex: {e}"))?;
        let hrp = Hrp::parse(DID_HRP).map_err(|e| format!("bad HRP: {e}"))?;
        bech32::encode::<Bech32m>(hrp, &bytes).map_err(|e| format!("bech32m encoding failed: {e}"))
    }

    /// `true` when `puzzle_hash` is the singleton launcher puzzle hash (any `0x` / case).
    pub fn is_singleton_launcher(puzzle_hash: &str) -> bool {
        puzzle_hash
            .trim_start_matches("0x")
            .eq_ignore_ascii_case(SINGLETON_LAUNCHER_PUZZLE_HASH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::keys::service::derive_wallet_keys_inner;
    use crate::test_util::deterministic_test_phrase;

    const DID_BECH32M: &str =
        "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
    const LAUNCHER_HEX: &str = "42fd7ee4b5735f88c58ff5ab6b3912216525e262bb99fa10dc66e4a3ec109c24";

    #[test]
    fn encodes_launcher_to_canonical_did() {
        assert_eq!(encode_did(LAUNCHER_HEX).unwrap(), DID_BECH32M);
    }

    #[test]
    fn launcher_puzzle_hash_matches_any_encoding() {
        let ph = "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";
        assert!(is_singleton_launcher(ph));
        assert!(is_singleton_launcher(&format!(
            "0x{}",
            ph.to_ascii_uppercase()
        )));
        assert!(!is_singleton_launcher(&"0".repeat(64)));
    }

    #[test]
    fn fingerprint_is_deterministic_and_matches_did_pk() {
        let keys = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        let other = derive_wallet_keys_inner(&deterministic_test_phrase()).unwrap();
        assert_eq!(wallet_fingerprint(&keys), wallet_fingerprint(&other));
        assert_eq!(wallet_fingerprint(&keys), keys.did_pk_hex());
    }
}
