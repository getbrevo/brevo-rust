use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BalanceClient {
    pub http_client: HttpClient,
}

impl BalanceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns Active Balances
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `limit` - Limit
    /// * `offset` - Offset
    /// * `sort_field` - Sort Field
    /// * `sort` - Sort Order
    /// * `contact_id` - Contact ID
    /// * `balance_definition_id` - Balance Definition ID
    /// * `include_internal` - Include balances tied to internal definitions.
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
    ///         .balance
    ///         .get_active_balances_api(
    ///             &"pid".to_string(),
    ///             &GetActiveBalancesApiQueryRequest {
    ///                 contact_id: 1000000,
    ///                 balance_definition_id: "balanceDefinitionId".to_string(),
    ///                 limit: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort: None,
    ///                 include_internal: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_active_balances_api(
        &self,
        pid: &str,
        request: &GetActiveBalancesApiQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLoyaltyBalanceProgramsPidActiveBalanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/balance/programs/{}/active-balance", pid),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("sortField", request.sort_field.clone())
                    .serialize("sort", request.sort.clone())
                    .int("contactId", request.contact_id.clone())
                    .string("balanceDefinitionId", request.balance_definition_id.clone())
                    .bool("includeInternal", request.include_internal.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns balance definition page
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `limit` - Limit the number of records returned
    /// * `offset` - Offset to paginate records
    /// * `sort_field` - Field to sort by
    /// * `sort` - Sort direction
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
    ///         .balance
    ///         .get_balance_definition_list(
    ///             &"pid".to_string(),
    ///             &GetBalanceDefinitionListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_balance_definition_list(
        &self,
        pid: &str,
        request: &GetBalanceDefinitionListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<GetBalanceDefinitionListResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/balance/programs/{}/balance-definitions", pid),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sortField", request.sort_field.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates balance definition and returns information
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
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
    ///         .balance
    ///         .create_balance_definition(
    ///             &"pid".to_string(),
    ///             &PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequest {
    ///                 name: "name".to_string(),
    ///                 unit: PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestUnit::Points,
    ///                 balance_availability_duration_modifier: None,
    ///                 balance_availability_duration_unit: None,
    ///                 balance_availability_duration_value: None,
    ///                 balance_expiration_date: None,
    ///                 balance_option_amount_overtaking_strategy: None,
    ///                 balance_option_credit_rounding: None,
    ///                 balance_option_debit_rounding: None,
    ///                 description: None,
    ///                 image_ref: None,
    ///                 max_amount: None,
    ///                 max_credit_amount_limit: None,
    ///                 max_debit_amount_limit: None,
    ///                 meta: None,
    ///                 min_amount: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_balance_definition(
        &self,
        pid: &str,
        request: &PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceDefinition, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/balance/programs/{}/balance-definitions", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns balance definition
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `bdid` - Balance Definition Id
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
    ///         .balance
    ///         .get_balance_definition(
    ///             &"pid".to_string(),
    ///             &"bdid".to_string(),
    ///             &GetBalanceDefinitionQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_balance_definition(
        &self,
        pid: &str,
        bdid: &str,
        request: &GetBalanceDefinitionQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceDefinition, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "loyalty/balance/programs/{}/balance-definitions/{}",
                    pid, bdid
                ),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates Balance definition
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `bdid` - Balance Definition Id
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
    ///         .balance
    ///         .update_balance_definition(
    ///             &"pid".to_string(),
    ///             &"bdid".to_string(),
    ///             &UpdateBalanceDefinitionRequest {
    ///                 name: "name".to_string(),
    ///                 unit: UpdateBalanceDefinitionRequestUnit::Points,
    ///                 balance_availability_duration_modifier: None,
    ///                 balance_availability_duration_unit: None,
    ///                 balance_availability_duration_value: None,
    ///                 balance_expiration_date: None,
    ///                 balance_option_amount_overtaking_strategy: None,
    ///                 balance_option_credit_rounding: None,
    ///                 balance_option_debit_rounding: None,
    ///                 description: None,
    ///                 image_ref: None,
    ///                 max_amount: None,
    ///                 max_credit_amount_limit: None,
    ///                 max_debit_amount_limit: None,
    ///                 meta: None,
    ///                 min_amount: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_balance_definition(
        &self,
        pid: &str,
        bdid: &str,
        request: &UpdateBalanceDefinitionRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceDefinition, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "loyalty/balance/programs/{}/balance-definitions/{}",
                    pid, bdid
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete Balance definition
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `bdid` - Balance Definition Id
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
    ///         .balance
    ///         .delete_balance_definition(&"pid".to_string(), &"bdid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_balance_definition(
        &self,
        pid: &str,
        bdid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "loyalty/balance/programs/{}/balance-definitions/{}",
                    pid, bdid
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates balance limit and sends the created UUID along with the data
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `bdid` - Balance Definition Id
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
    ///         .balance
    ///         .create_balance_limit(
    ///             &"pid".to_string(),
    ///             &"bdid".to_string(),
    ///             &CreateBalanceLimitRequest {
    ///                 constraint_type: CreateBalanceLimitRequestConstraintType::Transaction,
    ///                 duration_unit: CreateBalanceLimitRequestDurationUnit::Day,
    ///                 duration_value: 1,
    ///                 transaction_type: CreateBalanceLimitRequestTransactionType::Credit,
    ///                 value: 1,
    ///                 sliding_schedule: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        request: &CreateBalanceLimitRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceLimit, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "loyalty/balance/programs/{}/balance-definitions/{}/limits",
                    pid, bdid
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches balance limits and send the created UUID along with the data
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `bdid` - Balance Definition Id
    /// * `blid` - Balance Limit Id
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
    ///         .balance
    ///         .get_balance_limit(
    ///             &"pid".to_string(),
    ///             &"bdid".to_string(),
    ///             &"blid".to_string(),
    ///             &GetBalanceLimitQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        blid: &str,
        request: &GetBalanceLimitQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceLimit, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "loyalty/balance/programs/{}/balance-definitions/{}/limits/{}",
                    pid, bdid, blid
                ),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates balance limit
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `bdid` - Balance Definition Id
    /// * `blid` - Balance Limit Id
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
    ///         .balance
    ///         .update_balance_limit(
    ///             &"pid".to_string(),
    ///             &"bdid".to_string(),
    ///             &"blid".to_string(),
    ///             &UpdateBalanceLimitRequest {
    ///                 constraint_type: UpdateBalanceLimitRequestConstraintType::Transaction,
    ///                 duration_unit: UpdateBalanceLimitRequestDurationUnit::Day,
    ///                 duration_value: 1,
    ///                 transaction_type: UpdateBalanceLimitRequestTransactionType::Credit,
    ///                 value: 1,
    ///                 sliding_schedule: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        blid: &str,
        request: &UpdateBalanceLimitRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceLimit, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "loyalty/balance/programs/{}/balance-definitions/{}/limits/{}",
                    pid, bdid, blid
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete balance limit
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `bdid` - Balance Definition Id
    /// * `blid` - Balance Limit Id
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
    ///         .balance
    ///         .delete_balance_limit(
    ///             &"pid".to_string(),
    ///             &"bdid".to_string(),
    ///             &"blid".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_balance_limit(
        &self,
        pid: &str,
        bdid: &str,
        blid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "loyalty/balance/programs/{}/balance-definitions/{}/limits/{}",
                    pid, bdid, blid
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns contact balances for a given balance definition across all subscriptions.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `include_internal` - Include balances tied to internal definitions.
    /// * `limit` - Limit the number of records returned
    /// * `offset` - Skip a number of records
    /// * `sort` - Sort order
    /// * `sort_field` - Field to sort by
    /// * `balance_definition_id` - Balance Definition ID (required)
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
    ///         .balance
    ///         .get_contact_balances(
    ///             &"pid".to_string(),
    ///             &GetContactBalancesQueryRequest {
    ///                 balance_definition_id: "balanceDefinitionId".to_string(),
    ///                 include_internal: None,
    ///                 limit: None,
    ///                 offset: None,
    ///                 sort: None,
    ///                 sort_field: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_contact_balances(
        &self,
        pid: &str,
        request: &GetContactBalancesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetContactBalancesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/balance/programs/{}/contact-balances", pid),
                None,
                QueryBuilder::new()
                    .bool("includeInternal", request.include_internal.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("sortField", request.sort_field.clone())
                    .string("balanceDefinitionId", request.balance_definition_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns created order
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
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
    ///         .balance
    ///         .create_balance_order(
    ///             &"pid".to_string(),
    ///             &CreateBalanceOrderRequest {
    ///                 amount: 1.1,
    ///                 balance_definition_id: "balanceDefinitionId".to_string(),
    ///                 contact_id: 1,
    ///                 due_at: "dueAt".to_string(),
    ///                 source: CreateBalanceOrderRequestSource::Engine,
    ///                 expires_at: None,
    ///                 meta: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_balance_order(
        &self,
        pid: &str,
        request: &CreateBalanceOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateBalanceOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/balance/programs/{}/create-order", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns subscription balances
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `cid` - Contact Id
    /// * `include_internal` - Include balances tied to internal definitions.
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
    ///         .balance
    ///         .get_subscription_balances(
    ///             &"pid".to_string(),
    ///             &"cid".to_string(),
    ///             &GetSubscriptionBalancesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_subscription_balances(
        &self,
        pid: &str,
        cid: &str,
        request: &GetSubscriptionBalancesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSubscriptionBalancesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "loyalty/balance/programs/{}/subscriptions/{}/balances",
                    pid, cid
                ),
                None,
                QueryBuilder::new()
                    .bool("includeInternal", request.include_internal.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a balance for a contact
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `cid` - Contact Id
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
    ///         .balance
    ///         .create_subscription_balances(
    ///             &"pid".to_string(),
    ///             &"cid".to_string(),
    ///             &PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest {
    ///                 balance_definition_id: "balanceDefinitionId".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_subscription_balances(
        &self,
        pid: &str,
        cid: &str,
        request: &PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "loyalty/balance/programs/{}/subscriptions/{}/balances",
                    pid, cid
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns transaction history
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `limit` - Limit the number of records returned
    /// * `offset` - Page number to retrieve
    /// * `sort_field` - Field to sort by
    /// * `sort` - Sort order
    /// * `contact_id` - Contact ID
    /// * `balance_definition_id` - Balance Definition ID
    /// * `status` - Transaction status filter
    /// * `transaction_type` - Transaction type filter
    /// * `loyalty_subscription_id` - Loyalty Subscription ID filter
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
    ///         .balance
    ///         .get_transaction_history_api(
    ///             &"pid".to_string(),
    ///             &GetTransactionHistoryApiQueryRequest {
    ///                 contact_id: 1000000,
    ///                 balance_definition_id: "balanceDefinitionId".to_string(),
    ///                 limit: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort: None,
    ///                 status: None,
    ///                 transaction_type: None,
    ///                 loyalty_subscription_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_transaction_history_api(
        &self,
        pid: &str,
        request: &GetTransactionHistoryApiQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLoyaltyBalanceProgramsPidTransactionHistoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/balance/programs/{}/transaction-history", pid),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("sortField", request.sort_field.clone())
                    .serialize("sort", request.sort.clone())
                    .int("contactId", request.contact_id.clone())
                    .string("balanceDefinitionId", request.balance_definition_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("transactionType", request.transaction_type.clone())
                    .string(
                        "loyaltySubscriptionId",
                        request.loyalty_subscription_id.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Creates new transaction and returns information
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
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
    ///         .balance
    ///         .begin_transaction(
    ///             &"pid".to_string(),
    ///             &BeginTransactionRequest {
    ///                 amount: 1.1,
    ///                 balance_definition_id: "balanceDefinitionId".to_string(),
    ///                 transaction_type: None,
    ///                 contact_id: None,
    ///                 loyalty_subscription_id: None,
    ///                 meta: None,
    ///                 ttl: None,
    ///                 event_time: None,
    ///                 auto_complete: None,
    ///                 balance_expiry_in_minutes: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn begin_transaction(
        &self,
        pid: &str,
        request: &BeginTransactionRequest,
        options: Option<RequestOptions>,
    ) -> Result<Transaction, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/balance/programs/{}/transactions", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancels transaction
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `tid` - Transaction Id
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
    ///         .balance
    ///         .cancel_transaction(&"pid".to_string(), &"tid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_transaction(
        &self,
        pid: &str,
        tid: &str,
        options: Option<RequestOptions>,
    ) -> Result<Transaction, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "loyalty/balance/programs/{}/transactions/{}/cancel",
                    pid, tid
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Completes transaction
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program Id
    /// * `tid` - Transaction Id
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
    ///         .balance
    ///         .complete_transaction(&"pid".to_string(), &"tid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn complete_transaction(
        &self,
        pid: &str,
        tid: &str,
        options: Option<RequestOptions>,
    ) -> Result<Transaction, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "loyalty/balance/programs/{}/transactions/{}/complete",
                    pid, tid
                ),
                None,
                None,
                options,
            )
            .await
    }
}
