use crate::types::{
    rest,
    transaction::{response::TransactionResponse, Transaction},
};

#[derive(Debug, Clone)]
pub enum HTTPMethod {
    GET,
    POST,
}

/// A helper trait that provides endpoint name and type for request types and associates a particular response type.
/// If `HTTP_METHOD` is [HTTPMethod]::GET, the request type params are query fields.
/// If `HTTP_METHOD` is [HTTPMethod]::POST, request type should be serialized and used as request body payload.
pub trait RocketChainRequest: serde::Serialize {
    /// Response type returned by the corresponding endpoint.
    type Response: serde::de::DeserializeOwned;
    /// Endpoint name.
    const ENDPOINT: &str;
    /// HTTP method (GET/PUT).
    const HTTP_METHOD: HTTPMethod = HTTPMethod::GET;
}

impl RocketChainRequest for rest::account_fees::GetAccountFees {
    type Response = rest::account_fees::GetAccountFeesResponse;
    const ENDPOINT: &str = "account-fees";
}

impl RocketChainRequest for rest::account_nonce::GetAccountNonce {
    type Response = rest::account_nonce::GetAccountNonceResponse;
    const ENDPOINT: &str = "account-nonce";
}

impl RocketChainRequest for rest::assets::GetAssets {
    type Response = rest::assets::GetAssetsResponse;
    const ENDPOINT: &str = "assets";
}

impl RocketChainRequest for rest::bridge_events::GetBridgeEvents {
    type Response = rest::bridge_events::GetBridgeEventsResponse;
    const ENDPOINT: &str = "bridge-events";
}

impl RocketChainRequest for rest::candles::GetCandles {
    type Response = rest::candles::GetCandlesResponse;
    const ENDPOINT: &str = "candles";
}

impl RocketChainRequest for rest::collateral::GetCollateral {
    type Response = rest::collateral::GetCollateral;
    const ENDPOINT: &str = "collateral";
}

impl RocketChainRequest for rest::faucet_claim::GetFaucetClaim {
    type Response = rest::faucet_claim::GetFaucetClaimResponse;
    const ENDPOINT: &str = "faucet-claim";
}

impl RocketChainRequest for rest::fees::GetGlobalFees {
    type Response = rest::fees::GetGlobalFeesResponse;
    const ENDPOINT: &str = "fees";
}

impl RocketChainRequest for rest::funding_rate_events::GetFundingRateEvents {
    type Response = rest::funding_rate_events::GetFundingRateEventsResponse;
    const ENDPOINT: &str = "funding-rate-events";
}

impl RocketChainRequest for rest::instruments::GetInstruments {
    type Response = rest::instruments::GetInstrumentsResponse;
    const ENDPOINT: &str = "instruments";
}

impl RocketChainRequest for rest::max_leverage::GetMaxLeverage {
    type Response = rest::max_leverage::GetMaxLeverageResponse;
    const ENDPOINT: &str = "max-leverage";
}

impl RocketChainRequest for rest::open_orders::GetOpenOrders {
    type Response = rest::open_orders::GetOpenOrdersResponse;
    const ENDPOINT: &str = "open-orders";
}

impl RocketChainRequest for rest::order_events::GetAccountOrderEvents {
    type Response = rest::order_events::GetOrderEventsResponse;
    const ENDPOINT: &str = "order-events";
}

impl RocketChainRequest for rest::position_funding_events::GetAccountPositionFundingEvents {
    type Response = rest::position_funding_events::GetPositionFundingEventsResponse;
    const ENDPOINT: &str = "position-funding-events";
}

impl RocketChainRequest for rest::position::GetPosition {
    type Response = rest::position::GetPositionsResponse;
    const ENDPOINT: &str = "position";
}

impl RocketChainRequest for rest::vault_history::GetVaultHistory {
    type Response = rest::vault_history::GetVaultHistoryResponse;
    const ENDPOINT: &str = "vault-history";
}

impl RocketChainRequest for rest::vault_stats::GetVaultStats {
    type Response = rest::vault_stats::GetVaultStatsResponse;
    const ENDPOINT: &str = "vault-stats";
}

impl RocketChainRequest for rest::vaults::GetVaults {
    type Response = rest::vaults::GetVaultsResponse;
    const ENDPOINT: &str = "vaults";
}

impl RocketChainRequest for Transaction {
    type Response = TransactionResponse;
    const ENDPOINT: &str = "transaction";
    const HTTP_METHOD: HTTPMethod = HTTPMethod::POST;
}

impl RocketChainRequest for Vec<Transaction> {
    type Response = String;
    const ENDPOINT: &str = "batch_transactions";
    const HTTP_METHOD: HTTPMethod = HTTPMethod::POST;
}
