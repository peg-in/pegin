//! Browser wallet contexts: login signing, JWT, key derivation, and signing.
//! Chain reads moved to the relay (feat-37): the browser performs no chain I/O.
pub mod auth;
pub mod jwt;
pub mod keys;
pub mod signing;
