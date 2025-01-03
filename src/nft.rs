// nft.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NftContractMetadataRequest {
    pub contract_addresses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct NftContractMetadataResponse {
    pub metadata: Vec<NftContractMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct NftContractMetadata {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub token_type: String,
}

#[derive(Debug, Serialize)]
pub struct NftContractsByAccountRequest {
    pub account_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_addresses: Option<Vec<String>>,
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
pub struct NftMetadataByTokenIdsRequest {
    pub tokens: Vec<TokenIdentifier>,
}

#[derive(Debug, Serialize)]
pub struct NftTransfersByAccountRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_zero_value: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TokenIdentifier {
    pub contract_address: String,
    pub token_id: String,
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
