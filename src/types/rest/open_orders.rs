use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData, views::open_order::OpenOrderView,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GetOpenOrders {
    pub account: AccountAddress,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersResponse {
    pub orders: Option<Vec<OpenOrderView>>,
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
