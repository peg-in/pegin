//! DID string parsing for coinset lookups.

const DID_HRP: &str = "did:chia:";

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bech32m_did() {
        let did = "did:chia:1gt7hae94wd0c33v07k4kkwgjy9jjtcnzhwvl5yxuvmj28mqsnsjqvgw9uu";
        let hex = launcher_id_hex(did).expect("parse");
        assert_eq!(hex.len(), 64);
    }
}
