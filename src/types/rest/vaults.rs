use serde::{Deserialize, Serialize};

use crate::types::{rest::pagination::PaginationData, views::vault::VaultSetView};

pub type GetVaults = PaginationData;

#[derive(Serialize, Deserialize, Clone)]
pub struct GetVaultsResponse {
    pub vaults: VaultSetView,
}
