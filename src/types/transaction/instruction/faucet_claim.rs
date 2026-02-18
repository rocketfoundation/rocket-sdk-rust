use serde::{Deserialize, Serialize};

/// Data for a faucet claim transaction.
/// This transaction mints USDC (asset 0) to the sender's account.
/// Only available when testnet_faucet_mode is enabled.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct FaucetClaimData {}
