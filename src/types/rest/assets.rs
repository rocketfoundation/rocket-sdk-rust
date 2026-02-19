use serde::{Deserialize, Serialize};

use crate::types::{rest::pagination::PaginationData, views::assets::AssetSetView};

/// Request params for querying assets list.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAssets {
    #[serde(flatten)]
    pub pagination_data: PaginationData,
}

/// Response containing the list of assets.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAssetsResponse {
    /// Collection of asset data returned by the node.
    pub assets: AssetSetView,
}
