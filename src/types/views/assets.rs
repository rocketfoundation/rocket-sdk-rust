use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{macros::impl_as_ref_mut_newtype, types::primitives::AssetId};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AssetView {
    pub id: AssetId,
    pub ticker: String,
    pub haircut: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AssetSetView(HashMap<AssetId, AssetView>);

impl_as_ref_mut_newtype!(AssetSetView, HashMap<AssetId, AssetView>);
