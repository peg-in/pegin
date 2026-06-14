//! Challenge module: verifies the BLS signature over the login nonce.
#[path = "challenge.service.rs"]
mod challenge_service;

pub use challenge_service::verify_challenge_signature;
