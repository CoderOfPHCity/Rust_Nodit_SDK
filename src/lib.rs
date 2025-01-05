pub mod nft;
pub mod token;

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub use nft::{
    NftContractMetadataRequest, NftContractMetadataResponse, NftContractsByAccountRequest,
    NftMetadataByTokenIdsRequest, NftTransfersByAccountRequest, TokenIdentifier,
};
pub use token::{
    NativeBalanceRequest, NativeBalanceResponse, TokenAllowanceRequest, TokenAllowanceResponse,
    TokenHoldersRequest, TokenHoldersResponse, TokenMetadataRequest, TokenMetadataResponse,
    TokenPricesRequest, TokenPricesResponse, TokenTransfersRequest, TokenTransfersResponse,
};

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

    pub async fn get_native_balance_by_account(
        &self,
        account_address: &str,
    ) -> Result<NativeBalanceResponse> {
        let payload = NativeBalanceRequest {
            account_address: account_address.to_string(),
        };
        self.post(
            "/v1/kaia/mainnet/native/getNativeBalanceByAccount",
            &payload,
        )
        .await
    }

    pub async fn get_token_prices_by_contracts(
        &self,
        contract_addresses: Vec<String>,
        currency: Option<String>,
    ) -> Result<TokenPricesResponse> {
        let payload = TokenPricesRequest {
            contract_addresses,
            currency,
        };
        self.post("/v1/kaia/mainnet/token/getTokenPricesByContracts", &payload)
            .await
    }

    pub async fn get_token_transfers_by_account(
        &self,
        request: TokenTransfersRequest,
    ) -> Result<TokenTransfersResponse> {
        self.post(
            "/v1/kaia/mainnet/token/getTokenTransfersByAccount",
            &request,
        )
        .await
    }

    pub async fn get_token_allowance(
        &self,
        contract_address: &str,
        owner_address: &str,
        spender_address: &str,
    ) -> Result<TokenAllowanceResponse> {
        let payload = TokenAllowanceRequest {
            contract_address: contract_address.to_string(),
            owner_address: owner_address.to_string(),
            spender_address: spender_address.to_string(),
        };
        self.post("/v1/kaia/mainnet/token/getTokenAllowance", &payload)
            .await
    }

    pub async fn get_token_contract_metadata_by_contracts(
        &self,
        contract_addresses: Vec<String>,
    ) -> Result<TokenMetadataResponse> {
        let payload = TokenMetadataRequest { contract_addresses };
        self.post(
            "/v1/kaia/mainnet/token/getTokenContractMetadataByContracts",
            &payload,
        )
        .await
    }

    pub async fn get_token_holders_by_contract(
        &self,
        request: TokenHoldersRequest,
    ) -> Result<TokenHoldersResponse> {
        self.post("/v1/kaia/mainnet/token/getTokenHoldersByContract", &request)
            .await
    }

    pub async fn get_nft_contract_metadata_by_contracts(
        &self,
        contract_addresses: Vec<String>,
    ) -> Result<NftContractMetadataResponse> {
        let payload = NftContractMetadataRequest { contract_addresses };
        self.post(
            "/v1/kaia/mainnet/nft/getNftContractMetadataByContracts",
            &payload,
        )
        .await
    }

    pub async fn get_nft_contracts_by_account(
        &self,
        request: NftContractsByAccountRequest,
    ) -> Result<NftContractMetadataResponse> {
        self.post("/v1/kaia/mainnet/nft/getNftContractsByAccount", &request)
            .await
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
        self.post("/v1/kaia/mainnet/nft/getNftHoldersByContract", &payload)
            .await
    }

    pub async fn get_nft_metadata_by_token_ids(
        &self,
        tokens: Vec<TokenIdentifier>,
    ) -> Result<NftContractMetadataResponse> {
        let payload = NftMetadataByTokenIdsRequest { tokens };
        self.post("/v1/kaia/mainnet/nft/getNftMetadataByTokenIds", &payload)
            .await
    }

    pub async fn get_nft_transfers_by_account(
        &self,
        request: NftTransfersByAccountRequest,
    ) -> Result<TokenTransfersResponse> {
        self.post("/v1/kaia/mainnet/nft/getNftTransfersByAccount", &request)
            .await
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
        self.post(
            "/v1/kaia/mainnet/nft/searchNftContractMetadataByKeyword",
            &payload,
        )
        .await
    }
}
