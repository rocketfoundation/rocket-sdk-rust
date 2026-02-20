use crate::types::transaction::error::TxSerdeError;

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("Failed to sign message: {0}")]
    MessageSign(#[from] alloy_signer::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TxError {
    #[error("Failed to (de)serialize: {0}")]
    Serde(#[from] TxSerdeError),
    #[error("Sign error: {0}")]
    SignError(#[from] SignError),
}
