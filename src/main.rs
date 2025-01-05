use kaia_nodit::{NoditClient, TokenHoldersRequest, TokenTransfersRequest};

use std::env;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = env::var("NODIT_API_KEY").expect("NODIT_API_KEY must be set");

    let client = NoditClient::new(api_key)?;

    // Example wallet address and contract address
    let wallet_address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
    let contract_address = "0xdac17f958d2ee523a2206206994597c13d831ec7";

    // Test 1: Get native balance
    println!("Getting native balance...");
    let balance = client.get_native_balance_by_account(wallet_address).await?;
    println!("Native Balance: {} {}", balance.balance, balance.currency);

    // Test 2: Get token prices
    println!("\nGetting token prices...");
    let token_prices = client
        .get_token_prices_by_contracts(vec![contract_address.to_string()], Some("USD".to_string()))
        .await?;
    println!("Token Prices: {:?}", token_prices);

    // Test 3: Get token transfers
    println!("\nGetting token transfers...");
    let transfers_request = TokenTransfersRequest {
        account_address: wallet_address.to_string(),
        relation: None,
        contract_addresses: Some(vec![contract_address.to_string()]),
        from_block: None,
        to_block: None,
        from_date: None,
        to_date: None,
        page: Some(1),
        rpp: Some(10),
        cursor: None,
        with_count: Some(true),
    };
    let transfers = client
        .get_token_transfers_by_account(transfers_request)
        .await?;
    println!("Token Transfers: {:?}", transfers);

    // Test 4: Get token metadata
    println!("\nGetting token metadata...");
    let metadata = client
        .get_token_contract_metadata_by_contracts(vec![contract_address.to_string()])
        .await?;
    println!("Token Metadata: {:?}", metadata);

    // Test 5: Get token holders
    println!("\nGetting token holders...");
    let holders_request = TokenHoldersRequest {
        contract_address: contract_address.to_string(),
        page: Some(1),
        rpp: Some(10),
        cursor: None,
        with_count: Some(true),
    };
    let holders = client
        .get_token_holders_by_contract(holders_request)
        .await?;
    println!("Token Holders: {:?}", holders);

    Ok(())
}
