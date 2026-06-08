use pegin_domain::{did::Did, identity::Username};

// Literal is a hardcoded compile-time constant — panic is unreachable in practice.
#[allow(clippy::expect_used, clippy::missing_panics_doc)]
pub fn test_did() -> Did {
    Did::try_from("did:chia:deadbeef000000000000000000000000000000000000000000000000cafebabe")
        .expect("test DID is valid")
}

// Literal is a hardcoded compile-time constant — panic is unreachable in practice.
#[allow(clippy::expect_used, clippy::missing_panics_doc)]
pub fn test_username() -> Username {
    Username::try_from("alice").expect("test username is valid")
}
