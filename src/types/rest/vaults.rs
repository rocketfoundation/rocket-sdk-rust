use serde::{Deserialize, Serialize};

use crate::types::{rest::pagination::PaginationData, views::vault::VaultSetView};

/// Request parameters to get the list of existing vaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVaults {
    #[serde(flatten)]
    pagination_data: PaginationData,
}

/// Response containing a set of vaults.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetVaultsResponse {
    pub vaults: VaultSetView,
}
