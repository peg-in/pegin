//! DID identifier forms — bech32m `did:chia:1...` ⇄ 32-byte launcher ID hex.

use bech32::{Bech32m, Hrp};

/// Human-readable part of a Chia DID per chia-blockchain's bech32m encoding.
const DID_HRP: &str = "did:chia:";

/// Standard Chia singleton launcher puzzle hash — every DID launcher coin uses it.
const SINGLETON_LAUNCHER_PUZZLE_HASH: &str =
    "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";

/// Returns `true` when `puzzle_hash` is the singleton launcher puzzle hash.
/// Tolerates a `0x` prefix and any hex case — neither is contractual in coinset responses.
// Used only by the WASM on-chain check; kept cfg-free so `cargo test` covers it.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn is_singleton_launcher(puzzle_hash: &str) -> bool {
    puzzle_hash
        .trim_start_matches("0x")
        .eq_ignore_ascii_case(SINGLETON_LAUNCHER_PUZZLE_HASH)
}

/// Parses a DID identifier into the 32-byte launcher ID as lowercase hex.
///
/// * `input` — bech32m `did:chia:1...` (wallet display form) or 64 hex chars
pub fn parse_launcher_id(input: &str) -> Result<String, String> {
    if input.starts_with(DID_HRP) {
        let (hrp, bytes) =
            bech32::decode(input).map_err(|e| format!("invalid bech32m DID '{input}': {e}"))?;
        if hrp.as_str() != DID_HRP {
            return Err(format!("unexpected DID prefix '{}'", hrp.as_str()));
        }
        if bytes.len() != 32 {
            return Err(format!(
                "DID launcher ID must be 32 bytes, got {}",
                bytes.len()
            ));
        }
        Ok(hex::encode(bytes))
    } else if input.len() == 64 && input.chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(input.to_ascii_lowercase())
    } else {
        Err(format!(
            "expected 'did:chia:1...' or 64 hex chars, got '{input}'"
        ))
    }
}

/// Encodes a launcher ID (64-char hex) as the canonical bech32m DID string.
pub fn encode_did(launcher_id_hex: &str) -> Result<String, String> {
    let bytes = hex::decode(launcher_id_hex).map_err(|e| format!("invalid launcher hex: {e}"))?;
    let hrp = Hrp::parse(DID_HRP).map_err(|e| format!("bad HRP: {e}"))?;
    bech32::encode::<Bech32m>(hrp, &bytes).map_err(|e| format!("bech32m encoding failed: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real testnet11 DID created with the Sage wallet — known-good vector pair.
    const DID_BECH32M: &str =
        "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
    const LAUNCHER_HEX: &str = "42fd7ee4b5735f88c58ff5ab6b3912216525e262bb99fa10dc66e4a3ec109c24";

    #[test]
    fn parses_bech32m_did_to_launcher_hex() {
        assert_eq!(parse_launcher_id(DID_BECH32M).unwrap(), LAUNCHER_HEX);
    }

    #[test]
    fn parses_raw_hex_launcher_id() {
        assert_eq!(parse_launcher_id(LAUNCHER_HEX).unwrap(), LAUNCHER_HEX);
    }

    #[test]
    fn encode_decode_round_trip() {
        assert_eq!(encode_did(LAUNCHER_HEX).unwrap(), DID_BECH32M);
    }

    #[test]
    fn rejects_corrupted_bech32m_checksum() {
        let corrupted = format!("{}x", &DID_BECH32M[..DID_BECH32M.len() - 1]);
        assert!(parse_launcher_id(&corrupted).is_err());
    }

    #[test]
    fn rejects_garbage_input() {
        assert!(parse_launcher_id("not-a-did").is_err());
        assert!(parse_launcher_id("deadbeef").is_err());
    }

    #[test]
    fn launcher_puzzle_hash_matches_any_encoding() {
        let plain = "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";
        assert!(is_singleton_launcher(plain));
        assert!(is_singleton_launcher(&format!("0x{plain}")));
        assert!(is_singleton_launcher(&plain.to_ascii_uppercase()));
        assert!(!is_singleton_launcher(
            "0000000000000000000000000000000000000000000000000000000000000000"
        ));
    }
}
