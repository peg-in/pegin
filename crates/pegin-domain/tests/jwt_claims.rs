use pegin_domain::jwt_claims::JwtClaims;

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
