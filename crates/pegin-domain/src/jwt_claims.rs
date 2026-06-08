use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub preferred_username: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
}
