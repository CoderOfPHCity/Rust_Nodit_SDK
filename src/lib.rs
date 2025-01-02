use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use anyhow::Result;

// NFT-specific request/response structures
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
pub struct TokenIdentifier {
    pub contract_address: String,
    pub token_id: String,
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


#[derive(Debug, Serialize, Deserialize)]
pub struct NativeBalanceRequest {
    account_address: String,
}

#[derive(Debug, Deserialize)]
pub struct NativeBalanceResponse {
    pub balance: String,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPricesRequest {
    contract_addresses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
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

const BASE_URL: &str = "https://web3.nodit.io";

#[derive(Debug, Clone)]
pub struct NoditClient {
    client: Client,
    api_key: String,
}

impl NoditClient {
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_key: api_key.into(),
        })
    }

    pub async fn post<T, R>(&self, endpoint: &str, payload: &T) -> Result<R>
    where
        T: Serialize + std::fmt::Debug,
        R: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", BASE_URL, endpoint);
        
        let response = self
            .client
            .post(&url)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .header("X-API-KEY", &self.api_key)
            .json(payload)
            .send()
            .await
            .context("Failed to make HTTP request")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let result = response
            .json()
            .await
            .context("Failed to parse API response")?;

        Ok(result)
    }

    pub async fn get_native_balance_by_account(&self, account_address: &str) -> Result<NativeBalanceResponse> {
        let payload = NativeBalanceRequest {
            account_address: account_address.to_string(),
        };
        self.post("/v1/kaia/mainnet/native/getNativeBalanceByAccount", &payload).await
    }

    pub async fn get_token_prices_by_contracts(&self, contract_addresses: Vec<String>, currency: Option<String>) -> Result<TokenPricesResponse> {
        let payload = TokenPricesRequest {
            contract_addresses,
            currency,
        };
        self.post("/v1/kaia/mainnet/token/getTokenPricesByContracts", &payload).await
    }

    pub async fn get_token_transfers_by_account(&self, request: TokenTransfersRequest) -> Result<TokenTransfersResponse> {
        self.post("/v1/kaia/mainnet/token/getTokenTransfersByAccount", &request).await
    }

    pub async fn get_token_allowance(&self, contract_address: &str, owner_address: &str, spender_address: &str) -> Result<TokenAllowanceResponse> {
        let payload = TokenAllowanceRequest {
            contract_address: contract_address.to_string(),
            owner_address: owner_address.to_string(),
            spender_address: spender_address.to_string(),
        };
        self.post("/v1/kaia/mainnet/token/getTokenAllowance", &payload).await
    }

    pub async fn get_token_contract_metadata_by_contracts(&self, contract_addresses: Vec<String>) -> Result<TokenMetadataResponse> {
        let payload = TokenMetadataRequest {
            contract_addresses,
        };
        self.post("/v1/kaia/mainnet/token/getTokenContractMetadataByContracts", &payload).await
    }

    pub async fn get_token_holders_by_contract(&self, request: TokenHoldersRequest) -> Result<TokenHoldersResponse> {
        self.post("/v1/kaia/mainnet/token/getTokenHoldersByContract", &request).await
    }


    pub async fn get_nft_contract_metadata_by_contracts(
        &self,
        contract_addresses: Vec<String>,
    ) -> Result<NftContractMetadataResponse> {
        let payload = NftContractMetadataRequest { contract_addresses };
        self.post("/v1/kaia/mainnet/nft/getNftContractMetadataByContracts", &payload).await
    }

    pub async fn get_nft_contracts_by_account(
        &self,
        request: NftContractsByAccountRequest,
    ) -> Result<NftContractMetadataResponse> {
        self.post("/v1/kaia/mainnet/nft/getNftContractsByAccount", &request).await
    }

    pub async fn get_nft_holders_by_contract(
        &self,
        contract_address: &str,
        page: Option<i32>,
        rpp: Option<i32>,
        cursor: Option<String>,
        with_count: Option<bool>,
    ) -> Result<TokenHoldersResponse> {
        let payload = TokenHoldersRequest {
            contract_address: contract_address.to_string(),
            page,
            rpp,
            cursor,
            with_count,
        };
        self.post("/v1/kaia/mainnet/nft/getNftHoldersByContract", &payload).await
    }

    pub async fn get_nft_metadata_by_token_ids(
        &self,
        tokens: Vec<TokenIdentifier>,
    ) -> Result<NftContractMetadataResponse> {
        let payload = NftMetadataByTokenIdsRequest { tokens };
        self.post("/v1/kaia/mainnet/nft/getNftMetadataByTokenIds", &payload).await
    }

    pub async fn get_nft_transfers_by_account(
        &self,
        request: NftTransfersByAccountRequest,
    ) -> Result<TokenTransfersResponse> {
        self.post("/v1/kaia/mainnet/nft/getNftTransfersByAccount", &request).await
    }

    pub async fn search_nft_contract_metadata_by_keyword(
        &self,
        keyword: &str,
        page: Option<i32>,
        rpp: Option<i32>,
        cursor: Option<String>,
        with_count: Option<bool>,
    ) -> Result<NftContractMetadataResponse> {
        let payload = serde_json::json!({
            "keyword": keyword,
            "page": page,
            "rpp": rpp,
            "cursor": cursor,
            "withCount": with_count
        });
        self.post("/v1/kaia/mainnet/nft/searchNftContractMetadataByKeyword", &payload).await
    }
}