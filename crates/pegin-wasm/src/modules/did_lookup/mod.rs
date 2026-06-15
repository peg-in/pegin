//! Local DID lookup — resolve login identity from wallet keys.
//!
//! Key derivation, owner selection, JWT subject, and the profile cache live here.
//! On a cache miss the first login scans **public** coinset hints (`scan`/`coinset`)
//! to find the address index that owns the DID; secrets never leave WASM.

pub mod entities;
pub mod helper;
pub mod repository;
pub mod service;

// Public chain reads + the scan that uses them exist only in the browser (and tests).
#[cfg(any(target_arch = "wasm32", test))]
pub mod coinset;
#[cfg(any(target_arch = "wasm32", test))]
pub mod scan;

pub use entities::{ResolvedIdentity, DEFAULT_SCAN_LIMIT};
