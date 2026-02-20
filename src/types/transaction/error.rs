#[derive(Debug, thiserror::Error)]
pub enum TxSerdeError {
    #[cfg(feature = "json")]
    #[error("JSON: {0}")]
    JSON(#[from] serde_json::Error),
    #[cfg(feature = "messagepack")]
    #[error("MessagePack encode: {0}")]
    MessagePackEncode(#[from] rmp_serde::encode::Error),
    #[cfg(feature = "messagepack")]
    #[error("MessagePack decode: {0}")]
    MessagePackDecode(#[from] rmp_serde::decode::Error),
    #[error("Unsupported format, please check if all necessary features are enabled")]
    UnsupportedFormat,
}
