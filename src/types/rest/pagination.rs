use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationData {
    pub page_number: Option<usize>,
    pub page_size: Option<usize>,
}
