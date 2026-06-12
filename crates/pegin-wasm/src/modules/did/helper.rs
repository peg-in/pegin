//! DID identifier forms — bech32m `did:chia:1...` ⇄ 32-byte launcher ID hex.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use bech32::{Bech32m, Hrp};

/// Human-readable part of a Chia DID per chia-blockchain's bech32m encoding.
const DID_HRP: &str = "did:chia:";

/// Standard Chia singleton launcher puzzle hash — every DID launcher coin uses it.
const SINGLETON_LAUNCHER_PUZZLE_HASH: &str =
    "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";

/// Coinset REST bases this wallet is allowed to call (HTTPS only).
const ALLOWED_REST_BASES: &[&str] = &[
    "https://testnet11.api.coinset.org",
    "https://testnet.api.coinset.org",
    "https://api.coinset.org",
];

/// Default coinset WebSocket peer (feat-12); REST base is derived automatically.
pub const DEFAULT_PEER_WS: &str = "wss://testnet11.coinset.org:58444";

/// Default coinset REST API for testnet11.
pub const DEFAULT_REST_BASE: &str = "https://testnet11.api.coinset.org";

/// WebSocket peer hostnames mapped to their REST API base.
const WS_HOST_TO_REST: &[(&str, &str)] = &[
    ("testnet11.coinset.org", DEFAULT_REST_BASE),
    ("testnet.coinset.org", "https://testnet.api.coinset.org"),
    ("api.coinset.org", "https://api.coinset.org"),
    ("coinset.org", "https://api.coinset.org"),
];
/// Returns `true` when `puzzle_hash` is the singleton launcher puzzle hash.
/// Tolerates a `0x` prefix and any hex case — neither is contractual in coinset responses.
// Used only by the WASM on-chain check; kept cfg-free so `cargo test` covers it.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn is_singleton_launcher(puzzle_hash: &str) -> bool {
    puzzle_hash
        .trim_start_matches("0x")
        .eq_ignore_ascii_case(SINGLETON_LAUNCHER_PUZZLE_HASH)
}

/// Maps a peer URL (WebSocket or HTTPS) to an allowlisted coinset REST base URL.
pub fn rest_base_url(peer_url: Option<&str>) -> Result<String, String> {
    match peer_url {
        None => Ok(DEFAULT_REST_BASE.to_owned()),
        Some(url) if url.starts_with("https://") => parse_allowed_https(url),
        Some(url) if url.starts_with("wss://") => map_ws_peer(url),
        Some(url) if url.starts_with("ws://") => Err("peer URL must use wss://".to_owned()),
        Some(_) => Err("peer URL must use wss:// or https://".to_owned()),
    }
}

fn parse_allowed_https(url: &str) -> Result<String, String> {
    let trimmed = url.trim_end_matches('/');
    let host = trimmed
        .strip_prefix("https://")
        .and_then(|rest| rest.split('/').next())
        .ok_or_else(|| "invalid peer URL".to_owned())?;
    let base = format!("https://{host}");
    if ALLOWED_REST_BASES.contains(&base.as_str()) {
        Ok(base)
    } else {
        Err(format!("unsupported coinset peer: {host}"))
    }
}

fn map_ws_peer(url: &str) -> Result<String, String> {
    let host = url
        .trim_start_matches("wss://")
        .split(':')
        .next()
        .ok_or_else(|| "invalid peer URL".to_owned())?;
    WS_HOST_TO_REST
        .iter()
        .find(|(h, _)| *h == host)
        .map(|(_, rest)| (*rest).to_owned())
        .ok_or_else(|| format!("unsupported coinset peer: {host}"))
}

/// Parses a DID identifier into the 32-byte launcher ID as lowercase hex.
///
/// * `input` — bech32m `did:chia:1...` (wallet display form) or 64 hex chars
#[allow(dead_code)] // exercised in unit tests; reserved for future DID-id call sites
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
    fn rest_base_url_defaults_to_testnet11() {
        assert_eq!(rest_base_url(None).unwrap(), DEFAULT_REST_BASE);
    }

    #[test]
    fn rest_base_url_maps_ws_peer_to_https_api() {
        assert_eq!(rest_base_url(Some(DEFAULT_PEER_WS)).unwrap(), DEFAULT_REST_BASE);
        assert_eq!(
            rest_base_url(Some("https://testnet11.api.coinset.org")).unwrap(),
            "https://testnet11.api.coinset.org"
        );
    }

    #[test]
    fn rest_base_url_rejects_unknown_and_insecure_peers() {
        assert!(rest_base_url(Some("https://evil.example")).is_err());
        assert!(rest_base_url(Some("wss://evil.example:58444")).is_err());
        assert!(rest_base_url(Some("ws://testnet11.coinset.org:58444")).is_err());
        assert!(rest_base_url(Some("http://testnet11.api.coinset.org")).is_err());
    }

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
