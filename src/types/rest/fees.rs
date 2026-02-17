use serde::{Deserialize, Serialize};

use crate::types::views::global_fees::GlobalFeesClientView;

#[derive(Serialize, Deserialize)]
pub struct GetGlobalFeesResponse {
    pub result: GlobalFeesClientView,
}
