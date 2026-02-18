use crate::types::primitives::InstrumentIdError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("InstrumentId error: {0}")]
    InstrumentId(#[from] InstrumentIdError),
    #[error("Failed to deserialize: {0}")]
    Deserialize(#[from] DeserializeError),
    #[error("Failed to serialize: {0}")]
    Serialize(#[from] SerializeError),
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeError {
    #[cfg(feature = "json")]
    #[error("JSON: {0}")]
    JSON(#[from] serde_json::Error),
    #[cfg(feature = "messagepack")]
    #[error("MessagePack: {0}")]
    MessagePack(#[from] rmp_serde::decode::Error),
    #[error("Unsupported format, please check if all necessary features are enabled")]
    UnsupportedFormat,
}

#[derive(Debug, thiserror::Error)]
pub enum SerializeError {
    #[cfg(feature = "json")]
    #[error("JSON: {0}")]
    JSON(#[from] serde_json::Error),
    #[cfg(feature = "messagepack")]
    #[error("MessagePack: {0}")]
    MessagePack(#[from] rmp_serde::encode::Error),
    #[error("Unsupported format, please check if all necessary features are enabled")]
    UnsupportedFormat,
}
