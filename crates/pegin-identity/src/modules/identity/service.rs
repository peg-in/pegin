use pegin_domain::{did::Did, error::AppError, identity::Username};

pub trait IdentityStore: Send + Sync {
    fn find_by_username(&self, username: &Username) -> Result<Option<Did>, AppError>;
    fn save(&self, username: &Username, did: &Did) -> Result<(), AppError>;
}

pub trait PasskeyAuthenticator: Send + Sync {
    fn begin_registration(&self, did: &Did) -> Result<Vec<u8>, AppError>;
    fn finish_registration(&self, did: &Did, response: &[u8]) -> Result<(), AppError>;
    fn begin_authentication(&self, did: &Did) -> Result<Vec<u8>, AppError>;
    fn finish_authentication(&self, did: &Did, response: &[u8]) -> Result<(), AppError>;
}
