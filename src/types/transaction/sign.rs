use alloy_primitives::{keccak256, Address, Signature as AlloySignature, B256, B512};
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{sol, Eip712Domain, SolStruct};
use serde::{Deserialize, Serialize};

use crate::types::primitives::AccountAddress;

sol! {
    /// Minimal EIP-712 payload that signs the keccak256 of an arbitrary message.
    ///
    /// We intentionally keep the typed data compact and stable to avoid schema drift.
    struct RocketChainMessage {
        bytes32 contents;
    }
}

/// Returns the default EIP-712 domain used for Rocket Chain signatures.
fn rocket_chain_domain() -> Eip712Domain {
    Eip712Domain {
        name: Some("RocketChain".into()),
        version: Some("1".into()),
        chain_id: None,
        verifying_contract: None,
        salt: None,
    }
}

/// Compute the EIP-712 signing hash for an arbitrary message by wrapping the
/// keccak256 of the message bytes in a minimal typed-data struct.
pub fn eip712_signing_hash(message: &[u8]) -> B256 {
    let struct_hash = keccak256(message);
    let typed = RocketChainMessage {
        contents: struct_hash,
    };
    typed.eip712_signing_hash(&rocket_chain_domain())
}

/// Compute the EIP-191 message hash (with the Ethereum Signed Message prefix).
pub fn eip191_hash(message: &[u8]) -> B256 {
    let eip191_message = format!(
        "\x19Ethereum Signed Message:\n{}{}",
        message.len(),
        String::from_utf8_lossy(message)
    );
    keccak256(eip191_message.as_bytes())
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum SignatureScheme {
    #[default]
    Eip191,
    Eip712,
}

/// A cryptographic signature.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Signature(AlloySignature);

impl Default for Signature {
    fn default() -> Self {
        Signature(AlloySignature::from_raw_array(&[0u8; 65]).expect("Invalid signature"))
    }
}

impl Signature {
    /// Verify a signature with the specified scheme.
    pub fn verify_with_scheme(
        &self,
        message: &[u8],
        scheme: SignatureScheme,
        address: &AccountAddress,
    ) -> bool {
        match scheme {
            SignatureScheme::Eip191 => self.verify(message, address),
            SignatureScheme::Eip712 => self.verify_eip712(message, address),
        }
    }

    /// Convert the signature to bytes.
    pub fn _to_bytes(&self) -> [u8; 65] {
        self.0.as_bytes()
    }

    /// Create a signature from bytes.
    pub fn from_bytes(bytes: &[u8; 65]) -> Self {
        Signature(AlloySignature::from_raw_array(bytes).expect("Invalid signature"))
    }

    /// Verify a signature using EIP-191.
    pub fn verify(&self, message: &[u8], address: &AccountAddress) -> bool {
        let Ok(signer_address) = self.0.recover_address_from_msg(message) else {
            return false;
        };
        signer_address == address.0
    }

    /// Verify a signature using EIP-712.
    fn verify_eip712(&self, message: &[u8], address: &AccountAddress) -> bool {
        let digest = eip712_signing_hash(message);
        let Ok(signer_address) = self.0.recover_address_from_prehash(&digest) else {
            return false;
        };
        signer_address == address.0
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let hex_str = <&str>::deserialize(deserializer)?;
        let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let bytes = hex::decode(hex_str).map_err(serde::de::Error::custom)?;
        let signature = AlloySignature::from_raw(&bytes).map_err(serde::de::Error::custom)?;
        Ok(Signature(signature))
    }
}

/// An account signer that can sign messages.
#[derive(Clone)]
pub struct AccountSigner {
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
    pub fn from_hex_key(secret_key: String) -> Self {
        let secret_key = secret_key.trim().strip_prefix("0x").unwrap_or(&secret_key);
        let signer: PrivateKeySigner = secret_key.parse().unwrap();
        Self { signer }
    }

    /// Sign a message using EIP-191.
    pub fn sign(&mut self, message: &[u8]) -> Signature {
        self.sign_with_scheme(message, SignatureScheme::Eip191)
    }

    /// Sign a message with the specified scheme.
    pub fn sign_with_scheme(&mut self, message: &[u8], scheme: SignatureScheme) -> Signature {
        match scheme {
            SignatureScheme::Eip191 => {
                let signature = self
                    .signer
                    .sign_message_sync(message)
                    .expect("Failed to sign message");
                Signature(signature)
            }
            SignatureScheme::Eip712 => {
                let digest = eip712_signing_hash(message);
                let signature = self
                    .signer
                    .sign_hash_sync(&digest)
                    .expect("Failed to sign EIP-712 hash");
                Signature(signature)
            }
        }
    }
}
