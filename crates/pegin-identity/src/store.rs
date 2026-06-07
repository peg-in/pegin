use pegin_domain::{did::Did, error::AppError, identity::Username};

pub trait IdentityStore: Send + Sync {
    fn find_by_username(&self, username: &Username) -> Result<Option<Did>, AppError>;
    fn save(&self, username: &Username, did: &Did) -> Result<(), AppError>;
}
