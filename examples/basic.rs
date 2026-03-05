use std::time::Duration;

use rocket_chain_sdk::{
    client::{rest::RestClient, ws::WsClient},
    sign::AccountSigner,
    types::{
        primitives::{self, AccountAddress, InstrumentId},
        rest,
        transaction::{
            self,
            instruction::{self, order::OrderRequestSet, TransactionInstruction},
            response::TransactionResponse,
            RawTransaction, SerializationFormat,
        },
        ws::{
            client_message::ClientMessage, server_message::ServerMessage,
            subscription_kind::SubscriptionKind,
        },
    },
};

const REST_ENDPOINT_URL: &str = "http://127.0.0.1:3000";
const WS_ENDPOINT_URL: &str = "ws://127.0.0.1:4000";

fn incoming_ws_message_handler(message: ServerMessage) {
    println!("\nReceived server message: {message:?}");
}

#[tokio::main]
async fn main() {
    let private_key = std::env::var("ROCKET_PRIVATE_KEY")
        .expect("ROCKET_PRIVATE_KEY environment variable must be set");

    println!("Running example");

    let rest_client = RestClient::new(REST_ENDPOINT_URL).unwrap();
    let ws_client = WsClient::connect(WS_ENDPOINT_URL, incoming_ws_message_handler).unwrap();

    let mut signer = AccountSigner::from_hex_key(&private_key).unwrap();

    let mut nonce: u64 = 0;
    let mut next_nonce = || {
        nonce += 1;
        nonce
    };

    let instrument_id =
        send_mint_authority_transactions(&mut signer, &rest_client, &mut next_nonce).await;

    create_ws_subscriptions(&ws_client, signer.account_address(), instrument_id);
    tokio::time::sleep(Duration::from_secs(5)).await;

    send_user_transactions(&mut signer, &rest_client, &mut next_nonce, instrument_id).await;

    request_data(&rest_client, signer.account_address()).await;
    tokio::time::sleep(Duration::from_secs(20)).await;
}

async fn send_mint_authority_transactions(
    signer: &mut AccountSigner,
    rest_client: &RestClient,
    next_nonce: &mut impl FnMut() -> u64,
) -> InstrumentId {
    // List asset
    let list_asset_tx = RawTransaction {
        sender: signer.account_address(),
        nonce: next_nonce(),
        instruction: TransactionInstruction::ListAssets(instruction::list_asset::ListAssetsData {
            assets: vec![primitives::AssetRowData {
                ticker: "TEST".into(),
                haircut: 0,
                mark_price: 100,
                scenario_grid: [primitives::Scenario { price: 1, vol: 5 };
                    primitives::SCENARIO_COUNT],
                initial_scenario_grid: [primitives::Scenario { price: 1, vol: 5 };
                    primitives::SCENARIO_COUNT],
            }],
        }),
    }
    .sign(&SerializationFormat::JSON, signer)
    .unwrap();

    let TransactionResponse::ListAssets(response) =
        rest_client.send_request(list_asset_tx).await.unwrap()
    else {
        unreachable!();
    };
    println!("\nList asset response: {response:?}");

    let asset_id = response.assets[0].id;

    // List instrument
    let list_instrument_tx = RawTransaction {
        sender: signer.account_address(),
        nonce: next_nonce(),
        instruction: TransactionInstruction::ListInstruments(
            instruction::list_instrument::ListInstrumentsData {
                instruments: vec![primitives::InstrumentRowData {
                    underlying_asset_id: asset_id,
                    settlement_asset_id: asset_id,
                    expiry: 0,
                    strike: 0,
                    price_scale: 1000,
                    quantity_scale: 1000,
                    instrument_flags: primitives::PERPETUAL_INSTRUMENT,
                    is_trading: false,
                    pnl_grid: [0; primitives::SCENARIO_COUNT],
                    initial_pnl_grid: [0; primitives::SCENARIO_COUNT],
                }],
            },
        ),
    }
    .sign(&SerializationFormat::JSON, signer)
    .unwrap();

    let TransactionResponse::ListInstruments(response) =
        rest_client.send_request(list_instrument_tx).await.unwrap()
    else {
        unreachable!();
    };
    println!("\nList instrument response: {response:?}");

    let instrument_id = response.instruments[0].instrument_id;

    // Set is trading
    let set_is_trading_tx = RawTransaction {
        sender: signer.account_address(),
        nonce: next_nonce(),
        instruction: TransactionInstruction::SetIsTrading(
            instruction::set_is_trading::SetIsTradingData {
                instrument_id: instrument_id.clone(),
                is_trading: true,
            },
        ),
    }
    .sign(&SerializationFormat::JSON, signer)
    .unwrap();

    let response = rest_client.send_request(set_is_trading_tx).await.unwrap();
    println!("\nIs trading response: {response:?}");

    // Mint
    let mint_tx = RawTransaction {
        sender: signer.account_address(),
        nonce: next_nonce(),
        instruction: TransactionInstruction::Mint(instruction::mint::MintData {
            to: signer.account_address(),
            asset_id,
            amount: "10000000".to_string(),
        }),
    }
    .sign(&SerializationFormat::JSON, signer)
    .unwrap();

    let response = rest_client.send_request(mint_tx).await.unwrap();
    println!("\nMint response: {response:?}");

    instrument_id
}

fn create_ws_subscriptions(
    ws_client: &WsClient,
    account: primitives::AccountAddress,
    instrument_id: primitives::InstrumentId,
) {
    // Subscribe to order events for an account.
    ws_client.send(ClientMessage::Subscribe(SubscriptionKind::OrderEvents {
        account: Some(account.clone()),
        instrument_id: Some(instrument_id),
    }));

    // Subscribe to open orders for an account.
    ws_client.send(ClientMessage::Subscribe(SubscriptionKind::OpenOrders {
        account,
    }));

    // Subscribe to price feed for an instrument.
    ws_client.send(ClientMessage::Subscribe(SubscriptionKind::PriceFeed {
        instrument_id,
    }));
}

async fn send_user_transactions(
    signer: &mut AccountSigner,
    rest_client: &RestClient,
    next_nonce: &mut impl FnMut() -> u64,
    instrument_id: InstrumentId,
) {
    // Place limit order
    let place_order_tx = RawTransaction {
        sender: signer.account_address(),
        nonce: next_nonce(),
        instruction: TransactionInstruction::PlaceOrder(OrderRequestSet::from(vec![
            transaction::instruction::order::OrderRequest::Limit(
                instruction::order::PlaceLimitOrderRequest {
                    instrument_id,
                    side: instruction::order::OrderSide::Buy,
                    price: "10000".into(),
                    quantity: "1".into(),
                    trader: signer.account_address(),
                    trigger_price: None,
                    reduce_only: false,
                    take_profit: false,
                },
            ),
        ])),
    }
    .sign(&SerializationFormat::JSON, signer)
    .unwrap();

    let response = rest_client.send_request(place_order_tx).await.unwrap();
    println!("\nPlace order response: {response:?}");
}

async fn request_data(rest_client: &RestClient, account: AccountAddress) {
    // Get list of assets
    let assets_response = rest_client
        .send_request(rest::assets::GetAssets {
            pagination_data: rest::pagination::PaginationData {
                page_number: Some(0),
                page_size: Some(100),
            },
        })
        .await
        .unwrap();

    println!("\nAssets response: {assets_response:?}");

    // Get list of instruments
    let instruments_response = rest_client
        .send_request(rest::instruments::GetInstruments {
            pagination_data: rest::pagination::PaginationData {
                page_number: Some(0),
                page_size: Some(100),
            },
        })
        .await
        .unwrap();

    println!("\nInstruments response: {instruments_response:?}");

    // Get account's orders
    let open_orders_response = rest_client
        .send_request(rest::open_orders::GetOpenOrders {
            account,
            pagination_data: Default::default(),
        })
        .await
        .unwrap();

    println!("\nOpen orders response: {open_orders_response:?}");
}
