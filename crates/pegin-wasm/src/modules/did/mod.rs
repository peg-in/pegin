//! On-chain DID lookup via coinset.org (REST today, WebSocket peer later).

pub mod helper;
pub mod peer;
pub mod service;

pub use helper::DEFAULT_PEER_WS;
