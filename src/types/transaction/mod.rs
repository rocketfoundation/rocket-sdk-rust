pub mod error;
pub mod instruction;
pub mod response;
pub mod signature;

use std::cell::OnceCell;

use alloy_primitives::TxHash;
use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress,
    transaction::{
        error::TxSerdeError,
        instruction::TransactionInstruction,
        signature::{eip191_hash, eip712_signing_hash, Signature, SignatureScheme},
    },
};

/// Supported formats for serializing transactions.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum SerializationFormat {
    JSON,
    MessagePack,
}

/// Un-signed transaction data consisting of sender, instruction, and nonce.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RawTransaction {
    /// Address of the account sending the transaction.
    pub sender: AccountAddress,
    /// Instruction payload to execute.
    pub instruction: TransactionInstruction,
    /// Transaction nonce to prevent replay.
    pub nonce: u64,
}

/// Fully signed transaction ready for submission.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Raw transaction payload.
    pub data: RawTransaction,
    /// Format in which the transaction was serialized.
    pub serialization_format: SerializationFormat,
    /// Signature scheme used for this transaction (defaults to EIP-191 for compatibility).
    pub signature_scheme: SignatureScheme,
    /// Cryptographic signature over the serialized data.
    pub signature: Signature,
    /// Cached serialized bytes to avoid re-serializing on verify/hash.
    #[serde(skip)]
    pub serialized: OnceCell<Vec<u8>>,
}

impl Transaction {
    fn serialized_bytes(&self) -> Result<&[u8], TxSerdeError> {
        if let Some(cached) = self.serialized.get() {
            return Ok(cached.as_slice());
        }

        let serialized_data = self.data.serialize(&self.serialization_format)?;

        let _ = self.serialized.set(serialized_data.clone());
        Ok(self.serialized.get().unwrap().as_slice())
    }

    /// Get the serialized payload.
    pub fn serialize(&self) -> Result<Vec<u8>, TxSerdeError> {
        Ok(self.serialized_bytes()?.to_vec())
    }

    /// Verify signature. Returns `true` if the signature is valid.
    pub fn verify(&self) -> bool {
        let Ok(serialized) = self.serialized_bytes() else {
            return false;
        };
        self.signature
            .verify_with_scheme(serialized, self.signature_scheme, &self.data.sender)
    }

    /// Returns transaction hash.
    pub fn hash(&self) -> Result<TxHash, TxSerdeError> {
        let serialized = self.serialized_bytes()?;
        let digest = match self.signature_scheme {
            SignatureScheme::Eip191 => eip191_hash(serialized),
            SignatureScheme::Eip712 => eip712_signing_hash(serialized),
        };
        Ok(TxHash::from(digest))
    }
}

impl RawTransaction {
    /// Serialize the raw transaction.
    pub fn serialize(&self, format: &SerializationFormat) -> Result<Vec<u8>, TxSerdeError> {
        match format {
            #[cfg(feature = "json")]
            SerializationFormat::JSON => {
                let serialized = serde_json::to_vec(self)?;
                Ok(serialized)
            }
            #[cfg(feature = "messagepack")]
            SerializationFormat::MessagePack => {
                let serialized = rmp_serde::to_vec(self)?;
                Ok(serialized)
            }
            #[allow(unreachable_patterns)]
            _ => Err(TxSerdeError::UnsupportedFormat),
        }
    }

    /// Deserialize the raw transaciton.
    pub fn deserialize(
        serialized: &[u8],
        format: SerializationFormat,
    ) -> Result<Self, TxSerdeError> {
        match format {
            #[cfg(feature = "json")]
            SerializationFormat::JSON => {
                let deserialized: Self = serde_json::from_slice(serialized)?;
                Ok(deserialized)
            }
            #[cfg(feature = "messagepack")]
            SerializationFormat::MessagePack => {
                let deserialized: Self = rmp_serde::from_slice(serialized)?;
                Ok(deserialized)
            }
            #[allow(unreachable_patterns)]
            _ => Err(TxSerdeError::UnsupportedFormat),
        }
    }
}
