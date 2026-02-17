use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum InstrumentIdError {
    #[error("Failed to construct from hex: {0}")]
    FromHex(hex::FromHexError),
    #[error("Expected 8-byte instrument ID, got {0} bytes")]
    Len(usize),
}

/// An instrument identifier (8-byte hash).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct InstrumentId(pub u64);

impl InstrumentId {
    /// Convert to bytes.
    pub fn as_bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl From<u64> for InstrumentId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for InstrumentId {
    type Error = InstrumentIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.strip_prefix("0x").unwrap_or(value);
        let decoded = hex::decode(value).map_err(InstrumentIdError::FromHex)?;

        if decoded.len() != 8 {
            return Err(InstrumentIdError::Len(decoded.len()));
        }

        let mut array = [0u8; 8];
        array.copy_from_slice(&decoded);
        Ok(Self(u64::from_le_bytes(array)))
    }
}

impl fmt::Display for InstrumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.as_bytes()))
    }
}

impl fmt::Debug for InstrumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InstrumentId(0x{})", hex::encode(self.as_bytes()))
    }
}

impl Serialize for InstrumentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(self.as_bytes())))
    }
}

impl<'de> Deserialize<'de> for InstrumentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded: String = Deserialize::deserialize(deserializer)?;
        Self::try_from(encoded.as_str()).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
