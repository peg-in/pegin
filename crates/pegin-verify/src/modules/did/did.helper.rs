//! DID string parsing and encoding for coinset lookups.

use bech32::{Bech32m, Hrp};

const DID_HRP: &str = "did:chia:";

/// Standard Chia singleton launcher puzzle hash — every DID launcher coin uses it.
const SINGLETON_LAUNCHER_PUZZLE_HASH: &str =
    "eff07522495060c066f66f32acc2a77e3a3e737aca8baea4d1a64ea4cdc13da9";

/// Parses a `did:chia:…` string into lowercase launcher-id hex (64 chars).
pub fn launcher_id_hex(did: &str) -> Result<String, String> {
    let Some(body) = did.strip_prefix(DID_HRP) else {
        return Err(format!("expected bech32m 'did:chia:1…', got '{did}'"));
    };
    // Raw-hex form `did:chia:<64 hex>` — check before bech32m, which never yields
    // a 64-char all-hex body, so the two forms can't collide.
    if body.len() == 64 && body.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(body.to_ascii_lowercase());
    }
    let (hrp, bytes) =
        bech32::decode(did).map_err(|e| format!("invalid bech32m DID '{did}': {e}"))?;
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
}

/// Encodes a launcher ID (64-char hex) as the canonical bech32m `did:chia:1…` string.
pub fn encode_did(launcher_id_hex: &str) -> Result<String, String> {
    let bytes = hex::decode(launcher_id_hex).map_err(|e| format!("invalid launcher hex: {e}"))?;
    if bytes.len() != 32 {
        return Err(format!(
            "invalid launcher hex length: expected 32 bytes, got {}",
            bytes.len()
        ));
    }
    let hrp = Hrp::parse(DID_HRP).map_err(|e| format!("bad HRP: {e}"))?;
    bech32::encode::<Bech32m>(hrp, &bytes).map_err(|e| format!("bech32m encoding failed: {e}"))
}

/// `true` when `puzzle_hash` is the singleton launcher puzzle hash (any `0x`/`0X` prefix / case).
pub fn is_singleton_launcher(puzzle_hash: &str) -> bool {
    puzzle_hash
        .strip_prefix("0x")
        .or_else(|| puzzle_hash.strip_prefix("0X"))
        .unwrap_or(puzzle_hash)
        .eq_ignore_ascii_case(SINGLETON_LAUNCHER_PUZZLE_HASH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bech32m_did() {
        let did = "did:chia:1zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygsx2z7xu";
        let hex = launcher_id_hex(did).expect("parse");
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn encode_decode_round_trip() {
        let launcher = "1111111111111111111111111111111111111111111111111111111111111111";
        let did = "did:chia:1zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygsx2z7xu";
        assert_eq!(encode_did(launcher).expect("encode"), did);
    }
}
