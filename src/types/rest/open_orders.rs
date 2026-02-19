use serde::{Deserialize, Serialize};

use crate::types::{
    primitives::AccountAddress, rest::pagination::PaginationData, views::open_order::OpenOrderView,
};

/// Request params to fetch open orders for a given account.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOpenOrders {
    /// Account whose open orders are being queried.
    pub account: AccountAddress,
    /// Pagination parameters.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}

/// Response containing a list of open orders.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersResponse {
    /// Optional list of open order views.
    pub orders: Option<Vec<OpenOrderView>>,
    /// Pagination data.
    #[serde(flatten, default)]
    pub pagination_data: PaginationData,
}
