use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{impl_as_ref_mut_newtype, types::primitives::AssetId};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct CollateralView(HashMap<AssetId, String>);

impl_as_ref_mut_newtype!(CollateralView, HashMap<AssetId, String>);
