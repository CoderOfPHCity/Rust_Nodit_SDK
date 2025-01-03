// token.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NativeBalanceRequest {
    pub account_address: String,
}

#[derive(Debug, Deserialize)]
pub struct NativeBalanceResponse {
    pub balance: String,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPricesRequest {
    pub contract_addresses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenPricesResponse {
    pub prices: Vec<TokenPrice>,
}

#[derive(Debug, Deserialize)]
pub struct TokenPrice {
    pub contract_address: String,
    pub price: String,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct TokenAllowanceRequest {
    pub contract_address: String,
    pub owner_address: String,
    pub spender_address: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenAllowanceResponse {
    pub allowance: String,
}

#[derive(Debug, Serialize)]
pub struct TokenMetadataRequest {
    pub contract_addresses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenMetadataResponse {
    pub metadata: Vec<TokenMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct TokenMetadata {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
}

#[derive(Debug, Serialize)]
pub struct TokenTransfersRequest {
    pub account_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_count: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TokenHoldersRequest {
    pub contract_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_count: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TokenHoldersResponse {
    pub holders: Vec<TokenHolder>,
    pub paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct TokenHolder {
    pub address: String,
    pub balance: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenTransfersResponse {
    pub transfers: Vec<TokenTransfer>,
    pub paging: Paging,
}

#[derive(Debug, Deserialize)]
pub struct TokenTransfer {
    pub contract_address: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub block_number: u64,
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Paging {
    pub current_page: i32,
    pub total_pages: i32,
    pub total_items: i32,
}


