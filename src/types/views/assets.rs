use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{macros::impl_as_ref_mut_newtype, types::primitives::AssetId};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AssetView {
    /// Unique asset identifier.
    pub id: AssetId,
    /// Asset ticker symbol.
    pub ticker: String,
    /// Haircut value as a decimal string.
    pub haircut: String,
    /// Current mark price as a decimal string.
    #[serde(rename = "markPrice")]
    pub mark_price: String,
}

/// Collection of asset views indexed by asset ID.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AssetSetView(HashMap<AssetId, AssetView>);

impl_as_ref_mut_newtype!(AssetSetView, HashMap<AssetId, AssetView>);
