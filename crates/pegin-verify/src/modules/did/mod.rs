//! DID parsing, coinset lookups, and on-chain owner verification.
#[path = "did.helper.rs"]
mod did_helper;
#[path = "did.repository.rs"]
mod did_repository;
mod owner;

pub use did_helper::launcher_id_hex;
pub use did_repository::CoinsetClient;
pub use owner::{expected_owner_p2, is_owner, parse_owner_p2_hash, verify_did_owner, CoinParts};
