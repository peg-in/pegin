use pegin_domain::error::AppError;

pub struct ChiaGateway {
    pub endpoint: String,
}

impl ChiaGateway {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self { endpoint: endpoint.into() }
    }

    pub async fn submit_transaction(&self, _spend_bundle: &[u8]) -> Result<(), AppError> {
        todo!("ChiaGateway::submit_transaction — implemented in feat-8")
    }
}
