use pegin_domain::{did::Did, identity::Username};

pub fn test_did() -> Did {
    Did::new("did:chia:test000000000000000000000000000000000000000000000000000000000000")
}

pub fn test_username() -> Username {
    Username::new("alice")
}
