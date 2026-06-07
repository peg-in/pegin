use pegin_domain::{did::Did, error::AppError};

pub trait PasskeyAuthenticator: Send + Sync {
    fn begin_registration(&self, did: &Did) -> Result<Vec<u8>, AppError>;
    fn finish_registration(&self, did: &Did, response: &[u8]) -> Result<(), AppError>;
    fn begin_authentication(&self, did: &Did) -> Result<Vec<u8>, AppError>;
    fn finish_authentication(&self, did: &Did, response: &[u8]) -> Result<(), AppError>;
}
