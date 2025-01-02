# Nodit SDK for Rust

The Nodit SDK for Rust provides a structured and easy-to-use client to interact with the Nodit Web3 API, enabling blockchain developers to retrieve native balances, fetch token prices, and query token transfers for Ethereum accounts and contracts.

## Features

- Retrieve native balances by account address.
- Fetch token prices for specific contracts in various currencies.
- Query token transfers by account with flexible filtering options.

## Installation

- Ensure you have Rust installed on your system.
- Add the SDK to your Rust project by copying the code from rust_nodit_sdk.rs.

Include the required dependencies in your `Cargo.toml`:
```
[dependencies]
anyhow = "1.0"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

### Usage

1. Initialize the SDK

Create an instance of NoditSDK using your API key:

use nodit_sdk::NoditSDK;
```
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = "your_api_key_here";
    let sdk = NoditSDK::new(api_key)?;
    
    // Example usage
    Ok(())
}
```