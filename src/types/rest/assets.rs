use serde::{Deserialize, Serialize};

use crate::types::{rest::pagination::PaginationData, views::assets::AssetSetView};

pub type GetAssets = PaginationData;

#[derive(Serialize, Deserialize, Clone)]
pub struct GetAssetsResponse {
    pub assets: AssetSetView,
}
