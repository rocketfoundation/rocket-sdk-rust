use serde::{Deserialize, Serialize};

/// Coarse instrument classification used in account views.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AggregatedInstrumentType {
    Spot,
    Perpetuals,
    Futures,
    Options,
    #[default]
    Unknown,
}
