use pegin_domain::{did::Did, error::AppError};

pub struct AssertPasskey;

impl AssertPasskey {
    pub fn execute(&self, _did: &Did, _response: &[u8]) -> Result<(), AppError> {
        todo!("AssertPasskey use case — implemented in feat-7+")
    }
}
