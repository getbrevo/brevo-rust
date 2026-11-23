use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RewardClient {
    pub http_client: HttpClient,
}

impl RewardClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get code count
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `cpid` - Code Pool ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .get_code_count(&"pid".to_string(), &"cpid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_code_count(
        &self,
        pid: &str,
        cpid: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetCodeCountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "loyalty/offer/programs/{}/code-pools/{}/codes-count",
                    pid, cpid
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a reward page
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `limit` - Page size
    /// * `offset` - Pagination offset
    /// * `state` - State of the reward
    /// * `version` - Version
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .get_reward_page_api(
    ///             &"pid".to_string(),
    ///             &GetRewardPageApiQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_reward_page_api(
        &self,
        pid: &str,
        request: &GetRewardPageApiQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLoyaltyOfferProgramsPidOffersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/offer/programs/{}/offers", pid),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("state", request.state.clone())
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new reward in the loyalty program.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .create_reward(
    ///             &"pid".to_string(),
    ///             &CreateRewardRequest {
    ///                 name: "name".to_string(),
    ///                 public_description: None,
    ///                 public_image: None,
    ///                 public_name: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_reward(
        &self,
        pid: &str,
        request: &CreateRewardRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateRewardResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/offer/programs/{}/offers", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a voucher and attribute it to a specific membership.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .create_voucher(
    ///             &"pid".to_string(),
    ///             &CreateVoucherRequest {
    ///                 reward_id: "rewardId".to_string(),
    ///                 code: None,
    ///                 contact_id: None,
    ///                 expiration_date: None,
    ///                 loyalty_subscription_id: None,
    ///                 meta: None,
    ///                 value: None,
    ///                 valid_from: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_voucher(
        &self,
        pid: &str,
        request: &CreateVoucherRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateVoucherResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/offer/programs/{}/rewards/attribute", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a request to redeem a voucher.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .redeem_voucher(
    ///             &"pid".to_string(),
    ///             &RedeemVoucherRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn redeem_voucher(
        &self,
        pid: &str,
        request: &RedeemVoucherRequest,
        options: Option<RequestOptions>,
    ) -> Result<Redeem, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/offer/programs/{}/rewards/redeem", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Completes voucher redeem request.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `tid` - Redeem transaction ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .complete_redeem_transaction(&"pid".to_string(), &"tid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn complete_redeem_transaction(
        &self,
        pid: &str,
        tid: &str,
        options: Option<RequestOptions>,
    ) -> Result<Redeem, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "loyalty/offer/programs/{}/rewards/redeem/{}/complete",
                    pid, tid
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Revoke attributed vouchers.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `attributed_reward_ids` - Reward Attribution IDs (comma seperated)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .revoke_vouchers(
    ///             &"pid".to_string(),
    ///             &RevokeVouchersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn revoke_vouchers(
        &self,
        pid: &str,
        request: &RevokeVouchersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("loyalty/offer/programs/{}/rewards/revoke", pid),
                None,
                QueryBuilder::new()
                    .string("attributedRewardIds", request.attributed_reward_ids.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Validates a reward.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .validate_reward(
    ///             &"pid".to_string(),
    ///             &ValidateRewardRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn validate_reward(
        &self,
        pid: &str,
        request: &ValidateRewardRequest,
        options: Option<RequestOptions>,
    ) -> Result<ValidateRewardResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/offer/programs/{}/rewards/validate", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns reward information.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `rid` - Reward ID
    /// * `version` - Version
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .get_reward_information(
    ///             &"pid".to_string(),
    ///             &"rid".to_string(),
    ///             &GetRewardInformationQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_reward_information(
        &self,
        pid: &str,
        rid: &str,
        request: &GetRewardInformationQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/offer/programs/{}/rewards/{}", pid, rid),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get voucher for a contact
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `limit` - Page size
    /// * `offset` - Pagination offset
    /// * `sort` - Sort order
    /// * `sort_field` - Sort field
    /// * `contact_id` - Contact ID
    /// * `metadata_key_value` - Metadata value for a Key filter
    /// * `reward_id` - Reward ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reward
    ///         .get_voucher_for_a_contact(
    ///             &"pid".to_string(),
    ///             &GetVoucherForAContactQueryRequest {
    ///                 contact_id: 1,
    ///                 limit: None,
    ///                 offset: None,
    ///                 sort: None,
    ///                 sort_field: None,
    ///                 metadata_key_value: None,
    ///                 reward_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_voucher_for_a_contact(
        &self,
        pid: &str,
        request: &GetVoucherForAContactQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLoyaltyOfferProgramsPidVouchersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/offer/programs/{}/vouchers", pid),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("sortField", request.sort_field.clone())
                    .int("contactId", request.contact_id.clone())
                    .string("metadata_key_value", request.metadata_key_value.clone())
                    .string("rewardId", request.reward_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
