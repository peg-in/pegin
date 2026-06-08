use std::collections::HashMap;

use pegin_domain::{did::Did, error::AppError};
use pegin_identity::authenticator::PasskeyAuthenticator;

/// Implements [`PasskeyAuthenticator`] without any real `WebAuthn` ceremony.
///
/// Use [`MockPasskeyVerifier::with_credential`] to register test personas, then
/// call the trait methods to drive authentication flows in unit tests.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default)]
pub struct MockPasskeyVerifier {
    /// username → opaque credential id bytes
    credentials: HashMap<String, Vec<u8>>,
}

impl MockPasskeyVerifier {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a test persona by username. The credential id is deterministic.
    #[must_use]
    pub fn with_credential(mut self, username: &str) -> Self {
        self.credentials.insert(
            username.to_owned(),
            format!("{username}_mock_credential").into_bytes(),
        );
        self
    }
}

impl PasskeyAuthenticator for MockPasskeyVerifier {
    fn begin_registration(&self, _did: &Did) -> Result<Vec<u8>, AppError> {
        Ok(b"mock-registration-challenge".to_vec())
    }

    fn finish_registration(&self, _did: &Did, _response: &[u8]) -> Result<(), AppError> {
        Ok(())
    }

    fn begin_authentication(&self, _did: &Did) -> Result<Vec<u8>, AppError> {
        Ok(b"mock-authentication-challenge".to_vec())
    }

    fn finish_authentication(&self, _did: &Did, _response: &[u8]) -> Result<(), AppError> {
        Ok(())
    }
}
