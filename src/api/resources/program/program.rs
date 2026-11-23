use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ProgramClient {
    pub http_client: HttpClient,
}

impl ProgramClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns list of loyalty programs
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page
    /// * `sort_field` - Sort documents by field
    /// * `sort` - Sort order
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
    ///         .program
    ///         .get_lp_list(
    ///             &GetLpListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_lp_list(
        &self,
        request: &GetLpListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLpListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "loyalty/config/programs",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates loyalty program
    ///
    /// # Arguments
    ///
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
    ///         .program
    ///         .create_new_lp(
    ///             &CreateNewLpRequest {
    ///                 name: "name".to_string(),
    ///                 description: None,
    ///                 document_id: None,
    ///                 meta: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_new_lp(
        &self,
        request: &CreateNewLpRequest,
        options: Option<RequestOptions>,
    ) -> Result<LoyaltyProgram, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "loyalty/config/programs",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns loyalty program
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
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
    ///         .program
    ///         .get_loyalty_program_info(&"pid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_loyalty_program_info(
        &self,
        pid: &str,
        options: Option<RequestOptions>,
    ) -> Result<LoyaltyProgram, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/config/programs/{}", pid),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates loyalty program
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
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
    ///         .program
    ///         .update_loyalty_program(
    ///             &"pid".to_string(),
    ///             &UpdateLoyaltyProgramRequest {
    ///                 name: "name".to_string(),
    ///                 description: None,
    ///                 meta: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_loyalty_program(
        &self,
        pid: &str,
        request: &UpdateLoyaltyProgramRequest,
        options: Option<RequestOptions>,
    ) -> Result<LoyaltyProgram, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("loyalty/config/programs/{}", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes Loyalty Program
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
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
    ///         .program
    ///         .delete_loyalty_program(&"pid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_loyalty_program(
        &self,
        pid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("loyalty/config/programs/{}", pid),
                None,
                None,
                options,
            )
            .await
    }

    /// Partially updates loyalty program
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
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
    ///         .program
    ///         .partially_update_loyalty_program(
    ///             &"pid".to_string(),
    ///             &PartiallyUpdateLoyaltyProgramRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn partially_update_loyalty_program(
        &self,
        pid: &str,
        request: &PartiallyUpdateLoyaltyProgramRequest,
        options: Option<RequestOptions>,
    ) -> Result<LoyaltyProgram, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("loyalty/config/programs/{}", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Information of balances, tiers, rewards and subscription members for a subscription
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
    /// * `contact_id` - The contact ID to filter by.
    /// * `params` - A list of filter parameters for querying the subscription info.
    /// * `loyalty_subscription_id` - The loyalty subscription ID to filter by.
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
    ///         .program
    ///         .get_parameter_subscription_info(
    ///             &"pid".to_string(),
    ///             &GetParameterSubscriptionInfoQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_parameter_subscription_info(
        &self,
        pid: &str,
        request: &GetParameterSubscriptionInfoQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetParameterSubscriptionInfoResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/config/programs/{}/account-info", pid),
                None,
                QueryBuilder::new()
                    .string("contactId", request.contact_id.clone())
                    .string("params", request.params.clone())
                    .string(
                        "loyaltySubscriptionId",
                        request.loyalty_subscription_id.clone(),
                    )
                    .bool("includeInternal", request.include_internal.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete subscription for a contact
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
    /// * `cid` - Contact ID.
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
    ///         .program
    ///         .delete_contact_subscription(&"pid".to_string(), 1, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_contact_subscription(
        &self,
        pid: &str,
        cid: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("loyalty/config/programs/{}/contact/{}", pid, cid),
                None,
                None,
                options,
            )
            .await
    }

    /// Publishes loyalty program
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
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
    ///         .program
    ///         .publish_loyalty_program(&"pid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn publish_loyalty_program(
        &self,
        pid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/config/programs/{}/publish", pid),
                None,
                None,
                options,
            )
            .await
    }

    /// Add member to a subscription
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
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
    ///         .program
    ///         .subscribe_member_to_a_subscription(
    ///             &"pid".to_string(),
    ///             &SubscribeMemberToASubscriptionRequest {
    ///                 member_contact_ids: vec![1],
    ///                 contact_id: None,
    ///                 loyalty_subscription_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn subscribe_member_to_a_subscription(
        &self,
        pid: &str,
        request: &SubscribeMemberToASubscriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscribeMemberToASubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/config/programs/{}/subscription-members", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes member from a subscription
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
    /// * `member_contact_ids` - Comma-separated list of member contact IDs to delete from the subscription.
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
    ///         .program
    ///         .delete_contact_members(
    ///             &"pid".to_string(),
    ///             &DeleteContactMembersQueryRequest {
    ///                 member_contact_ids: "memberContactIds".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_contact_members(
        &self,
        pid: &str,
        request: &DeleteContactMembersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("loyalty/config/programs/{}/subscription-members", pid),
                None,
                QueryBuilder::new()
                    .string("memberContactIds", request.member_contact_ids.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Subscribes to a loyalty program
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID. A unique identifier for the loyalty program.
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
    ///         .program
    ///         .subscribe_to_loyalty_program(
    ///             &"pid".to_string(),
    ///             &SubscribeToLoyaltyProgramRequest {
    ///                 contact_id: 1000000,
    ///                 loyalty_subscription_id: None,
    ///                 creation_date: None,
    ///                 meta: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn subscribe_to_loyalty_program(
        &self,
        pid: &str,
        request: &SubscribeToLoyaltyProgramRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscribeToLoyaltyProgramResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/config/programs/{}/subscriptions", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
