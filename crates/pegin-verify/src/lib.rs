//! Relying-party verification — JWT, optional challenge, coinset DID anchor.

mod modules;
mod shared;

pub use modules::verify::{verify_login, VerifiedLogin, VerifyLoginInput};
pub use shared::error::VerifyError;

/// On-chain DID ownership verification (parses the DID singleton with chia-wallet-sdk).
pub mod did {
    pub use crate::modules::did::{
        expected_owner_p2, is_owner, parse_owner_p2_hash, verify_did_owner, CoinParts,
        CoinsetClient,
    };
}
