use serde::{Deserialize, Serialize};

/// OIDC-compatible claims embedded in a PEGIN session JWT.
///
/// Field names match the OIDC standard so the token is readable by
/// spec-compliant relying parties without custom claim mapping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject — the user's DID string (`did:chia:<launcher_id>`).
    pub sub: String,
    /// Human-readable display name.
    pub preferred_username: String,
    /// Audience — the relying party's `client_id` or origin.
    pub aud: String,
    /// Issued-at (Unix timestamp, seconds).
    pub iat: i64,
    /// Expiry (Unix timestamp, seconds).
    pub exp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips_through_serde() {
        let claims = JwtClaims {
            sub: "did:chia:deadbeef".to_owned(),
            preferred_username: "alice".to_owned(),
            aud: "https://example.com".to_owned(),
            iat: 1_000_000,
            exp: 1_003_600,
        };
        let json = serde_json::to_string(&claims).unwrap();
        let decoded: JwtClaims = serde_json::from_str(&json).unwrap();
        assert_eq!(claims, decoded);
    }
}
