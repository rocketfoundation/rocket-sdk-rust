use crate::types::primitives::InstrumentIdError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("InstrumentId error: {0}")]
    InstrumentId(#[from] InstrumentIdError),
}
