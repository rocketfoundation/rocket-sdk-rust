use std::ops::{Deref, DerefMut};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{
    macros::impl_as_ref_mut_newtype,
    types::primitives::{AccountAddress, GlobalOrderId, InstrumentId, MMPTag},
};

/// Maximum number of order requests accepted in a single `PlaceOrder`
/// transaction at intake (REST).
pub const MAX_ORDER_REQUESTS_PER_TRANSACTION: usize = 100;

/// Wrapper for a vector of order requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct OrderRequestSet(
    /// Underlying list of order requests.
    pub Vec<OrderRequest>,
);

impl Deref for OrderRequestSet {
    type Target = Vec<OrderRequest>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OrderRequestSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<OrderRequest>> for OrderRequestSet {
    fn from(vec: Vec<OrderRequest>) -> Self {
        OrderRequestSet(vec)
    }
}

impl From<OrderRequestSet> for Vec<OrderRequest> {
    fn from(set: OrderRequestSet) -> Self {
        set.0
    }
}

impl_as_ref_mut_newtype!(OrderRequestSet, Vec<OrderRequest>);

/// Variants representing different kinds of order actions.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrderRequest {
    /// Place a limit order.
    Limit(PlaceLimitOrderRequest),
    /// Place a market order.
    Market(PlaceMarketOrderRequest),
    /// Cancel a specific order.
    Cancel(CancelOrderRequest),
    /// Cancel all orders (optionally filtered).
    CancelAll(CancelAllOrderRequest),
    /// Modify an existing order.
    Modify(ModifyOrderRequest),
}

impl OrderRequest {
    /// Trader the request acts on.
    pub fn trader(&self) -> &AccountAddress {
        match self {
            OrderRequest::Limit(data) => &data.trader,
            OrderRequest::Market(data) => &data.trader,
            OrderRequest::Cancel(data) => &data.trader,
            OrderRequest::CancelAll(data) => &data.trader,
            OrderRequest::Modify(data) => &data.trader,
        }
    }

    /// Instrument id when the request targets a single instrument.
    pub fn instrument_id(&self) -> Option<InstrumentId> {
        match self {
            OrderRequest::Limit(data) => Some(data.instrument_id),
            OrderRequest::Market(data) => Some(data.instrument_id),
            _ => None,
        }
    }

    /// Optional MMP tag on place requests.
    pub fn mmp_tag(&self) -> Option<MMPTag> {
        match self {
            OrderRequest::Limit(data) => data.mmp_tag,
            OrderRequest::Market(data) => data.mmp_tag,
            OrderRequest::Cancel(_) | OrderRequest::CancelAll(_) | OrderRequest::Modify(_) => None,
        }
    }
}

/// Side of an order, either buy or sell.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Contract-type filter for [`CancelAllOrderRequest`]. `Option` matches both
/// calls and puts.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContractTypeFilter {
    Perp,
    Option,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceLimitOrderRequest {
    /// Identifier of the instrument.
    pub instrument_id: InstrumentId,
    /// Side of the order.
    pub side: OrderSide,
    /// Price at which to place the order (decimal string).
    pub price: String,
    /// Quantity to trade (decimal string).
    pub quantity: String,
    /// Address of the trader placing the order.
    pub trader: AccountAddress,
    /// Optional trigger price for conditional orders.
    pub trigger_price: Option<String>,
    /// If true, the order is reduce-only.
    pub reduce_only: bool,
    /// If true, the order is a take-profit order.
    pub take_profit: bool,
    /// Optional market-maker protection tag.
    #[serde(default)]
    pub mmp_tag: Option<MMPTag>,
}

impl Serialize for PlaceLimitOrderRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let include_mmp_tag = include_optional_field(serializer.is_human_readable(), &self.mmp_tag);
        let mut state = serializer
            .serialize_struct("PlaceLimitOrderRequest", 8 + usize::from(include_mmp_tag))?;
        state.serialize_field("instrumentId", &self.instrument_id)?;
        state.serialize_field("side", &self.side)?;
        state.serialize_field("price", &self.price)?;
        state.serialize_field("quantity", &self.quantity)?;
        state.serialize_field("trader", &self.trader)?;
        state.serialize_field("triggerPrice", &self.trigger_price)?;
        state.serialize_field("reduceOnly", &self.reduce_only)?;
        state.serialize_field("takeProfit", &self.take_profit)?;
        if include_mmp_tag {
            state.serialize_field("mmpTag", &self.mmp_tag)?;
        }
        state.end()
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceMarketOrderRequest {
    /// Identifier of the instrument.
    pub instrument_id: InstrumentId,
    /// Side of the order.
    pub side: OrderSide,
    /// Quantity to trade (decimal string).
    pub quantity: String,
    /// Address of the trader placing the order.
    pub trader: AccountAddress,
    /// Optional trigger price for conditional orders.
    pub trigger_price: Option<String>,
    /// If true, the order is reduce-only.
    pub reduce_only: bool,
    /// If true, the order is a take-profit order.
    pub take_profit: bool,
    /// Optional maximum slippage allowed (decimal string), default "0.03" (0.03%).
    pub max_slippage: Option<String>,
    /// Optional market-maker protection tag.
    #[serde(default)]
    pub mmp_tag: Option<MMPTag>,
}

impl Serialize for PlaceMarketOrderRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let include_mmp_tag = include_optional_field(serializer.is_human_readable(), &self.mmp_tag);
        let mut state = serializer
            .serialize_struct("PlaceMarketOrderRequest", 8 + usize::from(include_mmp_tag))?;
        state.serialize_field("instrumentId", &self.instrument_id)?;
        state.serialize_field("side", &self.side)?;
        state.serialize_field("quantity", &self.quantity)?;
        state.serialize_field("trader", &self.trader)?;
        state.serialize_field("triggerPrice", &self.trigger_price)?;
        state.serialize_field("reduceOnly", &self.reduce_only)?;
        state.serialize_field("takeProfit", &self.take_profit)?;
        state.serialize_field("maxSlippage", &self.max_slippage)?;
        if include_mmp_tag {
            state.serialize_field("mmpTag", &self.mmp_tag)?;
        }
        state.end()
    }
}

/// TWAP order request. Internally uses market order semantics (slippage-based
/// pricing) with quantity released gradually over `twap_interval` milliseconds.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTWAPRequest {
    /// Identifier of the instrument.
    pub instrument_id: InstrumentId,
    /// Side of the order.
    pub side: OrderSide,
    /// Quantity to trade (decimal string).
    pub quantity: String,
    /// Address of the trader placing the order.
    pub trader: AccountAddress,
    /// Optional maximum slippage allowed (decimal string).
    pub max_slippage: Option<String>,
    /// If true, the order is reduce-only.
    pub reduce_only: bool,
    /// TWAP interval in milliseconds (max 48h).
    pub twap_interval: u64,
    /// Step frequency in milliseconds (15_000..=3_600_000). Defaults to 15s when absent.
    #[serde(default)]
    pub frequency: Option<u64>,
    /// When true, step timing is randomized (deterministically) between 15s and 2min.
    #[serde(default)]
    pub randomize: Option<bool>,
    /// Optional market-maker protection tag.
    #[serde(default)]
    pub mmp_tag: Option<MMPTag>,
}

impl Serialize for PlaceTWAPRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let include_mmp_tag = include_optional_field(serializer.is_human_readable(), &self.mmp_tag);
        let mut state =
            serializer.serialize_struct("PlaceTWAPRequest", 9 + usize::from(include_mmp_tag))?;
        state.serialize_field("instrumentId", &self.instrument_id)?;
        state.serialize_field("side", &self.side)?;
        state.serialize_field("quantity", &self.quantity)?;
        state.serialize_field("trader", &self.trader)?;
        state.serialize_field("maxSlippage", &self.max_slippage)?;
        state.serialize_field("reduceOnly", &self.reduce_only)?;
        state.serialize_field("twapInterval", &self.twap_interval)?;
        state.serialize_field("frequency", &self.frequency)?;
        state.serialize_field("randomize", &self.randomize)?;
        if include_mmp_tag {
            state.serialize_field("mmpTag", &self.mmp_tag)?;
        }
        state.end()
    }
}

/// Modify an existing TWAP order's interval and/or remaining quantity.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifyTWAPRequest {
    /// Identifier of the TWAP order to modify.
    pub order_id: GlobalOrderId,
    /// Address of the trader modifying the order.
    pub trader: AccountAddress,
    /// New TWAP interval in milliseconds. If `None`, the interval is unchanged.
    pub new_twap_interval: Option<u64>,
    /// New total quantity (unsigned; the order's existing side is preserved).
    pub new_quantity: Option<String>,
    /// New step frequency in milliseconds. If `None`, unchanged.
    pub new_frequency: Option<u64>,
    /// New randomize setting. If `None`, unchanged.
    pub new_randomize: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Identifier of the order to cancel.
    #[serde(rename = "orderId")]
    pub order_id: GlobalOrderId,
    /// Address of the trader who owns the order.
    pub trader: AccountAddress,
}

/// Cancel every open order of `trader` that matches all of the supplied
/// filters. Absent filters match everything.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderRequest {
    /// Optional instrument filter for cancellation.
    pub instrument_id: Option<InstrumentId>,
    /// Address of the trader canceling orders.
    pub trader: AccountAddress,
    /// Underlying asset ticker, e.g. `BTC`. Matched case-insensitively.
    #[serde(default)]
    pub underlying: Option<String>,
    /// Optional contract-type filter (`perp` or `option`).
    #[serde(default)]
    pub contract_type: Option<ContractTypeFilter>,
    /// Inclusive lower absolute-delta bound. Only narrows option orders.
    #[serde(default)]
    pub delta_lower: Option<String>,
    /// Inclusive upper absolute-delta bound.
    #[serde(default)]
    pub delta_upper: Option<String>,
}

impl Serialize for CancelAllOrderRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let human = serializer.is_human_readable();
        let include_underlying = include_optional_field(human, &self.underlying);
        let include_contract_type = include_optional_field(human, &self.contract_type);
        let include_delta_lower = include_optional_field(human, &self.delta_lower);
        let include_delta_upper = include_optional_field(human, &self.delta_upper);
        let field_count = 2
            + usize::from(include_underlying)
            + usize::from(include_contract_type)
            + usize::from(include_delta_lower)
            + usize::from(include_delta_upper);
        let mut state = serializer.serialize_struct("CancelAllOrderRequest", field_count)?;
        state.serialize_field("instrumentId", &self.instrument_id)?;
        state.serialize_field("trader", &self.trader)?;
        if include_underlying {
            state.serialize_field("underlying", &self.underlying)?;
        }
        if include_contract_type {
            state.serialize_field("contractType", &self.contract_type)?;
        }
        if include_delta_lower {
            state.serialize_field("deltaLower", &self.delta_lower)?;
        }
        if include_delta_upper {
            state.serialize_field("deltaUpper", &self.delta_upper)?;
        }
        state.end()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderRequest {
    /// Identifier of the order to modify.
    pub order_id: GlobalOrderId,
    /// New price for the order (decimal string).
    pub new_price: String,
    /// Address of the trader modifying the order.
    pub trader: AccountAddress,
    /// New quantity for the order (decimal string).
    pub new_quantity: String,
    /// New optional trigger price.
    pub new_trigger_price: Option<String>,
}

/// JSON signing omits `None` optionals so frontend payloads round-trip.
/// MessagePack (and other binary formats) always write the field so archive
/// replay cannot drift.
fn include_optional_field<T>(human_readable: bool, value: &Option<T>) -> bool {
    !human_readable || value.is_some()
}

#[cfg(all(test, feature = "json"))]
mod tests {
    use super::*;

    fn sample_limit() -> PlaceLimitOrderRequest {
        PlaceLimitOrderRequest {
            instrument_id: InstrumentId(1),
            side: OrderSide::Buy,
            price: "1".into(),
            quantity: "1".into(),
            trader: AccountAddress::from([1u8; 20]),
            trigger_price: None,
            reduce_only: false,
            take_profit: false,
            mmp_tag: None,
        }
    }

    #[test]
    fn json_omits_none_mmp_tag() {
        let json = serde_json::to_string(&sample_limit()).unwrap();
        assert!(!json.contains("mmpTag"), "{json}");
    }
}
