# Rocket Chain Rust SDK

A lightweight Rust crate for interacting with a Rocket Chain node.

This crate provides request/response DTOs, basic clients for both HTTP REST and WebSocket interfaces, and some basic utils for signing transactions. It is designed to be minimalistic and modular.

Source, documentation and examples are available on the main repository: <https://github.com/rocketfoundation/rocket-sdk-rust>.

---

## Getting Started

Add the crate to your `Cargo.toml`: 
```toml
rocket-chain-sdk = "0.1"
``` 

Enable additional features if required:
```toml
rocket-chain-sdk = { version = "0.1", features = ["client", "sign"] }
```

Or get the barebone data transfer objects by disabling the default features:
```toml
rocket-chain-sdk = { version = "0.1", default-features = false }
```
---

## Cargo Features
The functionality is split into several Cargo features to keep dependencies optional and to let users pick only the pieces they need.

If no features are enabled (`default-features = false`), the crate exports barebone DTOs for REST and WS endpoints request and response types together with some utils for transaction serialization producing and verifying transaction signatures. All DTOs implement `Debug`, `Clone`, `serde::Serialize` and `sedre::Deserialize` for convenience.

Default features are `json` and `endpoints` (see below).

Use the `full` feature flag to get all the potentially useful funcitonality this crate provides.

`json` and `messagepack` features provide functionality for transaction (de)serialization in JSON and MessagePack formats.

The `endpoints` feature provides a `RocketChainRequest` trait that bounds together request and response types for REST along with corresponding endpoint names. It's designed to facilitate client development.

`sign` feature provides funcitonality for signing transactions using [PrivateKeySigner](https://docs.rs/alloy-signer-local/latest/alloy_signer_local/type.PrivateKeySigner.html) from the `alloy` crate.

`client` feature provides minimalistic reference implementations for REST and WS client using [`reqwest`](https://crates.io/crates/reqwest) and [`tungstenite`](https://crates.io/crates/tungstenite).

The `example` directory contains examples of sending various transactions, requesting data and creating data stream subscriptions.
