use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ExternalFeedsClient {
    pub http_client: HttpClient,
}

impl ExternalFeedsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves all external feeds from your Brevo account with filtering and pagination.
    ///
    /// **Use this to:**
    /// - Get an overview of all external data feeds
    /// - Find feeds by name using search functionality
    /// - Filter feeds by creation date range
    /// - Browse feeds by authentication type
    /// - Monitor feed library organization and usage
    ///
    /// **Key information returned:**
    /// - Feed details (UUID, name, URL, authentication type)
    /// - Feed configuration and settings
    /// - Creation and modification timestamps
    /// - Feed status and error information
    /// - Authentication and header configurations
    ///
    /// **Important considerations:**
    /// - External feeds enable dynamic content in email campaigns
    /// - Feeds must be accessible from Brevo servers
    /// - Authentication credentials are securely stored
    /// - Feed performance affects campaign delivery
    /// - Use pagination for accounts with many feeds
    /// - Date range filtering limited to 30 days maximum
    /// - Search functionality works on feed name matching
    /// - Internal feeds are system-managed and cannot be modified
    ///
    /// # Arguments
    ///
    /// * `search` - Can be used to filter records by search keyword on feed name
    /// * `start_date` - Mandatory if `endDate` is used. Starting date (YYYY-MM-DD) from which
    /// you want to fetch the list. Can be maximum 30 days older than current
    /// date.
    /// * `end_date` - Mandatory if `startDate` is used. Ending date (YYYY-MM-DD) till which
    /// you want to fetch the list. Maximum time period that can be selected is
    /// one month.
    /// * `sort` - Sort the results in the ascending/descending order of record creation.
    /// Default order is **descending** if `sort` is not passed.
    /// * `auth_type` - Filter the records by `authType` of the feed.
    /// * `limit` - Number of documents returned per page.
    /// * `offset` - Index of the first document on the page.
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
    ///         .external_feeds
    ///         .get_all_external_feeds(
    ///             &GetAllExternalFeedsQueryRequest {
    ///                 search: Some("product".to_string()),
    ///                 start_date: Some(NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap()),
    ///                 end_date: Some(NaiveDate::parse_from_str("2024-01-31", "%Y-%m-%d").unwrap()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_all_external_feeds(
        &self,
        request: &GetAllExternalFeedsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAllExternalFeedsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "feeds",
                None,
                QueryBuilder::new()
                    .string("search", request.search.clone())
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("authType", request.auth_type.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new external feed for dynamic content in email campaigns.
    ///
    /// **Use this to:**
    /// - Set up external data sources for dynamic content
    /// - Configure authentication for protected feeds
    /// - Enable real-time content updates in campaigns
    /// - Establish connections to product catalogs, blogs, or APIs
    ///
    /// **Key information returned:**
    /// - Created feed UUID for reference in campaigns
    /// - Success confirmation
    ///
    /// **Important considerations:**
    /// - Feed URL must be accessible from Brevo infrastructure
    /// - Authentication credentials are securely encrypted
    /// - Test feed accessibility before campaign use
    /// - Consider feed response time for campaign performance
    /// - Monitor feed reliability and uptime
    /// - Use caching for frequently accessed feeds
    /// - Maximum 5 retry attempts allowed for failed requests
    /// - Custom headers support for API integration requirements
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
    ///         .external_feeds
    ///         .create_external_feed(
    ///             &CreateExternalFeedRequest {
    ///                 name: "Public API Feed".to_string(),
    ///                 url: "https://jsonplaceholder.typicode.com/posts".to_string(),
    ///                 auth_type: Some(CreateExternalFeedRequestAuthType::NoAuth),
    ///                 max_retries: Some(3),
    ///                 cache: Some(true),
    ///                 username: None,
    ///                 password: None,
    ///                 token: None,
    ///                 headers: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_external_feed(
        &self,
        request: &CreateExternalFeedRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateExternalFeedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "feeds",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves details of a specific external feed by its UUID.
    ///
    /// **Use this to:**
    /// - Get complete configuration of an external feed
    /// - Check feed authentication settings
    /// - Review feed personalization options
    /// - Verify feed URL and parameters
    /// - Monitor feed modification history
    ///
    /// **Key information returned:**
    /// - Complete feed configuration and settings
    /// - Authentication credentials and headers
    /// - Personalization and fallback settings
    /// - Creation and modification timestamps
    /// - Cache and retry configurations
    ///
    /// **Important considerations:**
    /// - UUID must exist in your account
    /// - Provides complete feed information for troubleshooting
    /// - Essential before making modifications
    /// - Shows current feed health status
    /// - Useful for debugging feed issues
    ///
    /// # Arguments
    ///
    /// * `uuid` - UUID of the feed to fetch
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
    ///         .external_feeds
    ///         .get_external_feed_by_uuid(&"b1c2d3e4-f5a6-47b8-89c0-d1e2f3a4b5c6".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_external_feed_by_uuid(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetExternalFeedByUuidResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("feeds/{}", uuid), None, None, options)
            .await
    }

    /// Updates configuration of an existing external feed.
    ///
    /// **Use this to:**
    /// - Update feed URLs when data sources change
    /// - Modify authentication credentials
    /// - Change cache and retry settings
    /// - Update custom headers
    /// - Rename feeds for better organization
    ///
    /// **Key information returned:**
    /// - Success confirmation message
    ///
    /// **Important considerations:**
    /// - Only provided fields will be updated
    /// - Feed UUID must exist in your account
    /// - Authentication changes require verification
    /// - URL changes should be tested before campaign use
    /// - Monitor campaigns using this feed after updates
    ///
    /// # Arguments
    ///
    /// * `uuid` - UUID of the feed to update
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
    ///         .external_feeds
    ///         .update_external_feed(
    ///             &"b1c2d3e4-f5a6-47b8-89c0-d1e2f3a4b5c6".to_string(),
    ///             &UpdateExternalFeedRequest {
    ///                 name: Some("Updated Product Catalog".to_string()),
    ///                 url: Some("https://api.newstore.com/products/v2".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_external_feed(
        &self,
        uuid: &str,
        request: &UpdateExternalFeedRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("feeds/{}", uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an external feed from your Brevo account.
    ///
    /// **Use this to:**
    /// - Remove external feeds that are no longer needed
    /// - Clean up unused data sources
    /// - Remove test or outdated feeds
    /// - Maintain organized feed library
    ///
    /// **Key information returned:**
    /// - Success confirmation message
    ///
    /// **Important considerations:**
    /// - This action is PERMANENT and cannot be undone
    /// - Feed configuration and history will be lost
    /// - Check if feed is used in active campaigns before deletion
    /// - Remove feed references from email templates
    /// - Consider deactivating instead of deleting if unsure
    ///
    /// # Arguments
    ///
    /// * `uuid` - UUID of the feed to delete
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
    ///         .external_feeds
    ///         .delete_external_feed(&"b1c2d3e4-f5a6-47b8-89c0-d1e2f3a4b5c6".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_external_feed(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("feeds/{}", uuid),
                None,
                None,
                options,
            )
            .await
    }
}
