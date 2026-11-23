use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ConsentGroupsClient {
    pub http_client: HttpClient,
}

impl ConsentGroupsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of consent groups for the account.
    ///
    /// <Note>This endpoint is only available when the Consent Groups feature is enabled for your account. Returns `403` if the feature is not activated.</Note>
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of results to return (default 10, max 50)
    /// * `offset` - Number of results to skip (default 0)
    /// * `id` - Filter by consent group ID
    /// * `name` - Filter by name (case-insensitive partial match)
    /// * `signup_mode` - Filter by signup mode
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
    ///         .consent_groups
    ///         .get_consent_groups(
    ///             &GetConsentGroupsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_consent_groups(
        &self,
        request: &GetConsentGroupsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConsentGroupsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "contacts/consent-groups",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .int("id", request.id.clone())
                    .string("name", request.name.clone())
                    .serialize("signupMode", request.signup_mode.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new consent group for the account.
    ///
    /// <Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
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
    ///         .consent_groups
    ///         .create_consent_group(
    ///             &CreateConsentGroupRequest {
    ///                 name: "Newsletter EU".to_string(),
    ///                 signup_mode: CreateConsentGroupRequestSignupMode::Manual,
    ///                 description: None,
    ///                 list_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_consent_group(
        &self,
        request: &CreateConsentGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConsentGroup, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts/consent-groups",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single consent group by ID for the account.
    ///
    /// <Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the consent group
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
    ///     client.consent_groups.get_consent_group(1000000, None).await;
    /// }
    /// ```
    pub async fn get_consent_group(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<ConsentGroup, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("contacts/consent-groups/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates name, description, or signupMode of a consent group. At least one field must be provided.
    ///
    /// <Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the consent group to update
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
    ///         .consent_groups
    ///         .update_consent_group(
    ///             1000000,
    ///             &UpdateConsentGroupRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_consent_group(
        &self,
        id: i64,
        request: &UpdateConsentGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConsentGroup, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("contacts/consent-groups/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a consent group by ID and removes it from all associated contacts.
    ///
    /// <Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the consent group to delete
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
    ///         .consent_groups
    ///         .delete_consent_group(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_consent_group(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("contacts/consent-groups/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
