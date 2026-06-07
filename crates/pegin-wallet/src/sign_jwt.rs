use pegin_domain::{did::Did, error::AppError, identity::Sub};

pub struct SignJwt;

impl SignJwt {
    pub fn execute(&self, _did: &Did) -> Result<Sub, AppError> {
        todo!("SignJwt use case — implemented in feat-7+")
    }
}
