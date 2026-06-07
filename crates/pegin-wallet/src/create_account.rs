use pegin_domain::{did::Did, error::AppError, identity::Username};

pub struct CreateAccount;

impl CreateAccount {
    pub fn execute(&self, _username: &Username) -> Result<Did, AppError> {
        todo!("CreateAccount use case — implemented in feat-7+")
    }
}
