pub mod error;

use std::sync::OnceLock;

use alloy_primitives::{Address, B512};
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;

use crate::{
    sign::error::{SignError, TxError},
    types::{
        primitives::AccountAddress,
        transaction::{
            signature::{eip712_signing_hash, Signature, SignatureScheme},
            RawTransaction, SerializationFormat, Transaction,
        },
    },
};

/// An account signer that can sign messages.
#[derive(Clone, Debug)]
pub struct AccountSigner {
    /// Underlying private key signer.
    pub signer: PrivateKeySigner,
}

impl AccountSigner {
    /// Create a dummy signer for testing.
    pub fn dummy() -> Self {
        Self {
            signer: "0x1111111111111111111111111111111111111111111111111111111111111111"
                .to_string()
                .parse()
                .unwrap(),
        }
    }

    /// Get the public key.
    pub fn public_key(&self) -> B512 {
        self.signer.public_key()
    }

    /// Get the Ethereum address.
    pub fn address(&self) -> Address {
        self.signer.address()
    }

    /// Get the account address.
    pub fn account_address(&self) -> AccountAddress {
        AccountAddress::from(self.address())
    }

    /// Create a signer from a hex-encoded private key.
    pub fn from_hex_key(secret_key: &str) -> Result<Self, SignError> {
        let secret_key = secret_key.trim().strip_prefix("0x").unwrap_or(secret_key);
        let signer: PrivateKeySigner = secret_key
            .parse()
            .map_err(|e: alloy_signer_local::LocalSignerError| {
                SignError::KeyParse(e.to_string())
            })?;
        Ok(Self { signer })
    }

    /// Sign a message using EIP-191.
    pub fn sign(&mut self, message: &[u8]) -> Result<Signature, SignError> {
        self.sign_with_scheme(message, SignatureScheme::Eip191)
    }

    /// Sign a message with the specified scheme.
    pub fn sign_with_scheme(
        &mut self,
        message: &[u8],
        scheme: SignatureScheme,
    ) -> Result<Signature, SignError> {
        match scheme {
            SignatureScheme::Eip191 => {
                let signature = self.signer.sign_message_sync(message)?;

                Ok(Signature::from(signature))
            }
            SignatureScheme::Eip712 => {
                let digest = eip712_signing_hash(message);
                let signature = self.signer.sign_hash_sync(&digest)?;

                Ok(Signature::from(signature))
            }
        }
    }
}

impl From<PrivateKeySigner> for AccountSigner {
    fn from(signer: PrivateKeySigner) -> Self {
        Self { signer }
    }
}

impl RawTransaction {
    /// Produces a full signed transaction using Eip191 scheme.
    pub fn sign(
        &self,
        format: &SerializationFormat,
        signer: &mut AccountSigner,
    ) -> Result<Transaction, TxError> {
        self.sign_with_scheme(format, signer, SignatureScheme::Eip191)
    }

    /// Serialize and sign the raw transaction.
    pub fn sign_with_scheme(
        &self,
        format: &SerializationFormat,
        signer: &mut AccountSigner,
        scheme: SignatureScheme,
    ) -> Result<Transaction, TxError> {
        let serialized = self.serialize(format)?;
        let signature = signer.sign_with_scheme(&serialized, scheme)?;
        Ok(Transaction {
            data: self.clone(),
            serialization_format: format.clone(),
            signature_scheme: scheme,
            signature,
            serialized: OnceLock::from(serialized),
        })
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::TxHash;

    use crate::types::transaction::instruction::{mint::MintData, TransactionInstruction};

    use super::*;

    /// Creates a test signer with a known private key
    fn test_signer() -> AccountSigner {
        AccountSigner::from_hex_key(
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        )
        .unwrap()
    }

    /// Creates a different signer to simulate wrong signer
    fn wrong_signer() -> AccountSigner {
        AccountSigner::from_hex_key(
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
        )
        .unwrap()
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
        let wrong_signature = wrong_signer.sign(&serialized).unwrap();

        let tx_with_wrong_sig = Transaction {
            data: raw_tx,
            serialization_format: SerializationFormat::JSON,
            signature_scheme: SignatureScheme::Eip191,
            signature: wrong_signature,
            serialized: OnceLock::new(),
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
            serialized: OnceLock::new(),
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
            serialized: OnceLock::new(),
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
