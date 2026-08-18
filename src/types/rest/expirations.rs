use serde::{Deserialize, Serialize};

/// Request params for unique instrument expirations on the node.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExpirations {
    /// Comma-separated contract type filter. Accepted values
    /// (case-insensitive): `option`, `future`, or both (e.g. `option,future`).
    pub contract_type: String,
    /// Underlying asset ticker (required, case-insensitive), e.g. `BTC`.
    pub underlying_asset: String,
}

/// Sorted unique expiry labels for matching instruments.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetExpirationsResponse {
    /// Formatted as `DMMMYY`, e.g. `28MAR25` (day is not zero-padded).
    pub expirations: Vec<String>,
}
