use serde::{Deserialize, Serialize};

/// Common pagination parameters for REST requests.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaginationData {
    /// Optional page number to request, default `0`.
    pub page_number: Option<usize>,
    /// Optional size of each page, default `100`.
    pub page_size: Option<usize>,
}
