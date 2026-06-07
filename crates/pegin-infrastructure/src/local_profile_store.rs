use pegin_domain::{did::Did, error::AppError, identity::Username};
use pegin_identity::store::IdentityStore;

pub struct LocalProfileStore;

impl IdentityStore for LocalProfileStore {
    fn find_by_username(&self, _username: &Username) -> Result<Option<Did>, AppError> {
        todo!("LocalProfileStore — implemented in feat-7+")
    }

    fn save(&self, _username: &Username, _did: &Did) -> Result<(), AppError> {
        todo!("LocalProfileStore — implemented in feat-7+")
    }
}
