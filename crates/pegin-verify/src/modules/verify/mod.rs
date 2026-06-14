//! Verify module: the login verification use case (JWT + challenge + DID owner).
#[path = "verify.service.rs"]
mod verify_service;

pub use verify_service::{verify_login, VerifiedLogin, VerifyLoginInput};
