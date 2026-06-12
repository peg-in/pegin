#[path = "verify.service.rs"]
mod verify_service;

pub use verify_service::{verify_login, VerifiedLogin, VerifyLoginInput};
