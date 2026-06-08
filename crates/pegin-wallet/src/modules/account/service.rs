use pegin_domain::{
    did::Did,
    error::AppError,
    identity::{Sub, Username},
};

pub struct CreateAccount;

impl CreateAccount {
    pub fn execute(&self, _username: &Username) -> Result<Did, AppError> {
        todo!("CreateAccount use case — implemented in feat-7+")
    }
}

pub struct SignJwt;

impl SignJwt {
    pub fn execute(&self, _did: &Did) -> Result<Sub, AppError> {
        todo!("SignJwt use case — implemented in feat-7+")
    }
}

pub struct AssertPasskey;

impl AssertPasskey {
    pub fn execute(&self, _did: &Did, _response: &[u8]) -> Result<(), AppError> {
        todo!("AssertPasskey use case — implemented in feat-7+")
    }
}
