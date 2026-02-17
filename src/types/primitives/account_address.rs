use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash, str::FromStr};

/// An Ethereum-style account address (20 bytes).
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct AccountAddress(pub Address);

impl Serialize for AccountAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for AccountAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded: String = Deserialize::deserialize(deserializer)?;
        let decoded = Address::from_str(&encoded).map_err(serde::de::Error::custom)?;
        Ok(AccountAddress(decoded))
    }
}

impl From<Address> for AccountAddress {
    fn from(address: Address) -> Self {
        AccountAddress(address)
    }
}

impl From<&str> for AccountAddress {
    fn from(address: &str) -> Self {
        AccountAddress(Address::from_str(address).unwrap())
    }
}

impl From<[u8; 20]> for AccountAddress {
    fn from(address: [u8; 20]) -> Self {
        AccountAddress(Address::from(address))
    }
}

impl AccountAddress {
    /// Convert the address to little-endian bytes.
    pub fn to_le_bytes(&self) -> [u8; 20] {
        self.0.into_array()
    }
}

impl Display for AccountAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for AccountAddress {
    fn default() -> Self {
        AccountAddress(Address::ZERO)
    }
}
