pub mod instruction;
pub mod response;
pub mod sign;

use std::cell::OnceCell;

use alloy_primitives::TxHash;
use serde::{Deserialize, Serialize};

use crate::{
    error::*,
    types::{
        primitives::AccountAddress,
        transaction::{
            instruction::TransactionInstruction,
            sign::{eip191_hash, eip712_signing_hash, AccountSigner, Signature, SignatureScheme},
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum SerializationFormat {
    JSON,
    MessagePack,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RawTransaction {
    pub sender: AccountAddress,
    pub instruction: TransactionInstruction,
    pub nonce: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub data: RawTransaction,
    pub serialization_format: SerializationFormat,
    /// Signature scheme used for this transaction (defaults to EIP-191 for compatibility).
    pub signature_scheme: SignatureScheme,
    pub signature: Signature,
    /// Cached serialized bytes to avoid re-serializing on verify/hash.
    #[serde(skip)]
    pub serialized: OnceCell<Vec<u8>>,
}

impl Transaction {
    fn serialized_bytes(&self) -> Result<&[u8], Error> {
        if let Some(cached) = self.serialized.get() {
            return Ok(cached.as_slice());
        }

        let serialized_data = self.data.serialize(&self.serialization_format)?;

        let _ = self.serialized.set(serialized_data.clone());
        Ok(self.serialized.get().unwrap().as_slice())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(self.serialized_bytes()?.to_vec())
    }

    pub fn verify(&self) -> bool {
        let Ok(serialized) = self.serialized_bytes() else {
            return false;
        };
        self.signature
            .verify_with_scheme(serialized, self.signature_scheme, &self.data.sender)
    }

    pub fn hash(&self) -> Result<TxHash, Error> {
        let serialized = self.serialized_bytes()?;
        let digest = match self.signature_scheme {
            SignatureScheme::Eip191 => eip191_hash(serialized),
            SignatureScheme::Eip712 => eip712_signing_hash(serialized),
        };
        Ok(TxHash::from(digest))
    }
}

impl RawTransaction {
    pub fn serialize(&self, format: &SerializationFormat) -> Result<Vec<u8>, Error> {
        match format {
            #[cfg(feature = "json")]
            SerializationFormat::JSON => {
                let serialized = serde_json::to_vec(self).map_err(SerializeError::JSON)?;
                Ok(serialized)
            }
            #[cfg(feature = "messagepack")]
            SerializationFormat::MessagePack => {
                let serialized = rmp_serde::to_vec(self).map_err(SerializeError::MessagePack)?;
                Ok(serialized)
            }
            #[allow(unreachable_patterns)]
            _ => Err(SerializeError::UnsupportedFormat.into()),
        }
    }

    pub fn deserialize(serialized: &[u8], format: SerializationFormat) -> Result<Self, Error> {
        match format {
            #[cfg(feature = "json")]
            SerializationFormat::JSON => {
                let deserialized: Self =
                    serde_json::from_slice(serialized).map_err(DeserializeError::JSON)?;
                Ok(deserialized)
            }
            #[cfg(feature = "messagepack")]
            SerializationFormat::MessagePack => {
                let deserialized: Self =
                    rmp_serde::from_slice(serialized).map_err(DeserializeError::MessagePack)?;
                Ok(deserialized)
            }
            #[allow(unreachable_patterns)]
            _ => Err(DeserializeError::UnsupportedFormat.into()),
        }
    }

    pub fn sign(
        &self,
        format: &SerializationFormat,
        signer: &mut AccountSigner,
    ) -> Result<Transaction, Error> {
        self.sign_with_scheme(format, signer, SignatureScheme::Eip191)
    }

    pub fn sign_with_scheme(
        &self,
        format: &SerializationFormat,
        signer: &mut AccountSigner,
        scheme: SignatureScheme,
    ) -> Result<Transaction, Error> {
        let serialized = self.serialize(format)?;
        let signature = signer.sign_with_scheme(&serialized, scheme);
        Ok(Transaction {
            data: self.clone(),
            serialization_format: format.clone(),
            signature_scheme: scheme,
            signature,
            serialized: OnceCell::from(serialized),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::transaction::instruction::mint::MintData;

    /// Creates a test signer with a known private key
    fn test_signer() -> AccountSigner {
        AccountSigner::from_hex_key(
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
        )
    }

    /// Creates a different signer to simulate wrong signer
    fn wrong_signer() -> AccountSigner {
        AccountSigner::from_hex_key(
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".to_string(),
        )
    }

    fn create_test_raw_tx(sender: &AccountAddress) -> RawTransaction {
        RawTransaction {
            sender: sender.clone(),
            instruction: TransactionInstruction::Mint(MintData {
                to: sender.clone(),
                asset_id: 0, // USDC asset id
                amount: "1000".to_string(),
            }),
            nonce: 1,
        }
    }

    #[test]
    fn test_valid_signature_verification_succeeds() {
        let mut signer = test_signer();
        let sender = signer.account_address();
        let raw_tx = create_test_raw_tx(&sender);

        let signed_tx = raw_tx
            .sign(&SerializationFormat::JSON, &mut signer)
            .unwrap();

        // Valid signature should verify successfully
        assert!(signed_tx.verify(), "Valid signature should verify");
    }

    #[test]
    fn test_invalid_signature_verification_fails() {
        let correct_signer = test_signer();
        let mut wrong_signer = wrong_signer();
        let sender = correct_signer.account_address();
        let raw_tx = create_test_raw_tx(&sender);

        // Sign with wrong signer (different private key)
        let serialized = raw_tx.serialize(&SerializationFormat::JSON).unwrap();
        let wrong_signature = wrong_signer.sign(&serialized);

        let tx_with_wrong_sig = Transaction {
            data: raw_tx,
            serialization_format: SerializationFormat::JSON,
            signature_scheme: SignatureScheme::Eip191,
            signature: wrong_signature,
            serialized: OnceCell::new(),
        };

        // Invalid signature should fail verification
        assert!(
            !tx_with_wrong_sig.verify(),
            "Invalid signature should fail verification"
        );
    }

    #[test]
    fn test_tampered_transaction_fails_verification() {
        let mut signer = test_signer();
        let sender = signer.account_address();
        let raw_tx = create_test_raw_tx(&sender);

        let signed_tx = raw_tx
            .sign(&SerializationFormat::JSON, &mut signer)
            .unwrap();

        // Tamper with the transaction data (change nonce)
        let tampered_tx = Transaction {
            data: RawTransaction {
                sender: sender.clone(),
                instruction: TransactionInstruction::Mint(MintData {
                    to: sender.clone(),
                    asset_id: 0, // USDC asset id
                    amount: "1000".to_string(),
                }),
                nonce: 999, // Changed nonce!
            },
            serialization_format: signed_tx.serialization_format,
            signature_scheme: signed_tx.signature_scheme,
            signature: signed_tx.signature,
            serialized: OnceCell::new(),
        };

        // Tampered transaction should fail verification
        assert!(
            !tampered_tx.verify(),
            "Tampered transaction should fail verification"
        );
    }

    #[test]
    fn test_default_signature_fails_verification() {
        let signer = test_signer();
        let sender = signer.account_address();
        let raw_tx = create_test_raw_tx(&sender);

        // Transaction with default (zero) signature
        let tx_with_default_sig = Transaction {
            data: raw_tx,
            serialization_format: SerializationFormat::JSON,
            signature_scheme: SignatureScheme::Eip191,
            signature: Signature::default(),
            serialized: OnceCell::new(),
        };

        // Default signature should fail verification
        assert!(
            !tx_with_default_sig.verify(),
            "Default signature should fail verification"
        );
    }

    #[test]
    fn test_signature_with_messagepack_format() {
        let mut signer = test_signer();
        let sender = signer.account_address();
        let raw_tx = create_test_raw_tx(&sender);

        let signed_tx = raw_tx
            .sign(&SerializationFormat::MessagePack, &mut signer)
            .unwrap();

        // Valid signature with MessagePack should also verify
        assert!(
            signed_tx.verify(),
            "Valid MessagePack signature should verify"
        );
    }

    #[test]
    fn test_eip712_signature_verification_succeeds() {
        let mut signer = test_signer();
        let sender = signer.account_address();
        let raw_tx = create_test_raw_tx(&sender);

        let signed_tx = raw_tx
            .sign_with_scheme(
                &SerializationFormat::JSON,
                &mut signer,
                SignatureScheme::Eip712,
            )
            .unwrap();

        assert!(signed_tx.verify(), "Valid EIP-712 signature should verify");

        // Ensure the hashing path matches the EIP-712 digest
        let hash = signed_tx.hash().expect("hash should compute");
        let serialized = signed_tx.serialize().unwrap();
        let digest = eip712_signing_hash(&serialized);
        assert_eq!(hash, TxHash::from(digest));
    }
}
