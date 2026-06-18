//! Chain gateway seam (feat-37) — the relay's swap point for chain reads.
//!
//! `/resolve` and `/session` depend only on [`ChainResolver`]; today the sole impl is
//! [`CoinsetResolver`] (coinset.org REST). A future lite-wallet backend (feat-25) drops in
//! behind the same trait without touching the auth routes.

#[path = "coinset.resolver.rs"]
mod coinset_resolver;
mod resolver;

pub use coinset_resolver::CoinsetResolver;
pub use resolver::{account_pk_from_hex, ChainResolver, ResolveError};
