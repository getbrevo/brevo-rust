use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TierClient {
    pub http_client: HttpClient,
}

impl TierClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Manually assigns a tier to a specific membership.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `cid` - Contact ID
    /// * `tid` - Tier ID
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
    ///         .tier
    ///         .add_subscription_to_tier(
    ///             &"pid".to_string(),
    ///             &"cid".to_string(),
    ///             &"tid".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_subscription_to_tier(
        &self,
        pid: &str,
        cid: &str,
        tid: &str,
        options: Option<RequestOptions>,
    ) -> Result<AddSubscriptionToTierResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "loyalty/tier/programs/{}/contacts/{}/tiers/{}",
                    pid, cid, tid
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the list of tier groups defined within the loyalty program.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `version` - Select 'active' to retrieve list of all tier groups which are live for clients. Select draft to retrieve list of all non deleted tier groups.
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
    ///         .tier
    ///         .get_list_of_tier_groups(
    ///             &"pid".to_string(),
    ///             &GetListOfTierGroupsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_list_of_tier_groups(
        &self,
        pid: &str,
        request: &GetListOfTierGroupsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetListOfTierGroupsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/tier/programs/{}/tier-groups", pid),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new tier group in a loyalty program. *(The changes will take effect with the next publication of the loyalty program)*
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
    ///         .tier
    ///         .create_tier_group(
    ///             &"pid".to_string(),
    ///             &CreateTierGroupRequest {
    ///                 name: "name".to_string(),
    ///                 downgrade_strategy: None,
    ///                 meta: None,
    ///                 tier_order: None,
    ///                 upgrade_strategy: None,
    ///                 upgrade_schedule: None,
    ///                 downgrade_schedule: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_tier_group(
        &self,
        pid: &str,
        request: &CreateTierGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<TierGroup, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/tier/programs/{}/tier-groups", pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns tier group information.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `gid` - Tier group ID
    /// * `version` - Select active to retrieve active version of tier group. Select draft to retrieve latest changes in tier group.
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
    ///         .tier
    ///         .get_tier_group(
    ///             &"pid".to_string(),
    ///             &"gid".to_string(),
    ///             &GetTierGroupQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_tier_group(
        &self,
        pid: &str,
        gid: &str,
        request: &GetTierGroupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TierGroup, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/tier/programs/{}/tier-groups/{}", pid, gid),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a tier group from a loyalty program. *(The changes will take effect with the next publication of the loyalty program)*
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `gid` - Tier group ID
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
    ///         .tier
    ///         .update_tier_group(
    ///             &"pid".to_string(),
    ///             &"gid".to_string(),
    ///             &UpdateTierGroupRequest {
    ///                 downgrade_strategy: UpdateTierGroupRequestDowngradeStrategy::RealTime,
    ///                 name: "name".to_string(),
    ///                 tier_order: vec!["tierOrder".to_string()],
    ///                 upgrade_strategy: UpdateTierGroupRequestUpgradeStrategy::RealTime,
    ///                 meta: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_tier_group(
        &self,
        pid: &str,
        gid: &str,
        request: &UpdateTierGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<TierGroup, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("loyalty/tier/programs/{}/tier-groups/{}", pid, gid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a tier group from a loyalty program. *(The changes will take effect with the next publication of the loyalty program)*
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `gid` - Tier group ID
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
    ///         .tier
    ///         .delete_tier_group(&"pid".to_string(), &"gid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_tier_group(
        &self,
        pid: &str,
        gid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("loyalty/tier/programs/{}/tier-groups/{}", pid, gid),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a new tier in a loyalty program tier group. *(The changes will take effect with the next publication of the loyalty program)*
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `gid` - Tier group ID
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
    ///         .tier
    ///         .create_tier_for_tier_group(
    ///             &"pid".to_string(),
    ///             &"gid".to_string(),
    ///             &CreateTierForTierGroupRequest {
    ///                 access_conditions: vec![CreateTierForTierGroupRequestAccessConditionsItem {
    ///                     ..Default::default()
    ///                 }],
    ///                 name: "name".to_string(),
    ///                 image_ref: None,
    ///                 tier_rewards: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_tier_for_tier_group(
        &self,
        pid: &str,
        gid: &str,
        request: &CreateTierForTierGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<Tier, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("loyalty/tier/programs/{}/tier-groups/{}/tiers", pid, gid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the list of tiers defined within the loyalty program.
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `version` - Select 'active' to retrieve list of all tiers which are live for clients. Select draft to retrieve list of all non deleted tiers.
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
    ///         .tier
    ///         .get_loyalty_program_tier(
    ///             &"pid".to_string(),
    ///             &GetLoyaltyProgramTierQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_loyalty_program_tier(
        &self,
        pid: &str,
        request: &GetLoyaltyProgramTierQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLoyaltyProgramTierResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("loyalty/tier/programs/{}/tiers", pid),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Modifies an existing tier for the specified tier group *(The changes will take effect with the next publication of the loyalty program)*
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `tid` - Tier ID
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
    ///         .tier
    ///         .update_tier(
    ///             &"pid".to_string(),
    ///             &"tid".to_string(),
    ///             &UpdateTierRequest {
    ///                 access_conditions: vec![UpdateTierRequestAccessConditionsItem {
    ///                     ..Default::default()
    ///                 }],
    ///                 name: "name".to_string(),
    ///                 tier_rewards: vec![UpdateTierRequestTierRewardsItem {
    ///                     ..Default::default()
    ///                 }],
    ///                 image_ref: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_tier(
        &self,
        pid: &str,
        tid: &str,
        request: &UpdateTierRequest,
        options: Option<RequestOptions>,
    ) -> Result<Tier, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("loyalty/tier/programs/{}/tiers/{}", pid, tid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a tier from a loyalty program tier group. *(The changes will take effect with the next publication of the loyalty program)*
    ///
    /// # Arguments
    ///
    /// * `pid` - Loyalty Program ID
    /// * `tid` - Tier ID
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
    ///         .tier
    ///         .delete_tier(&"pid".to_string(), &"tid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_tier(
        &self,
        pid: &str,
        tid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("loyalty/tier/programs/{}/tiers/{}", pid, tid),
                None,
                None,
                options,
            )
            .await
    }
}
