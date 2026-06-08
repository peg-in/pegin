use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub preferred_username: String,
    pub aud: String,
    pub iat: i64,
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
