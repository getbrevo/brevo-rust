use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves all webhooks from your Brevo account with filtering and sorting options.
    ///
    /// Use this to:
    /// - Monitor webhook configurations and event handling
    /// - List webhooks by type (transactional, marketing, inbound)
    /// - Review webhook endpoints and authentication
    /// - Track webhook creation and modification history
    /// - Audit webhook event subscriptions
    ///
    /// Key information returned:
    /// - Complete webhook details and configuration
    /// - Event types and channel subscriptions
    /// - Authentication and security settings
    /// - Webhook URLs and custom headers
    /// - Creation and modification timestamps
    ///
    /// # Arguments
    ///
    /// * `type_` - Filter on webhook type
    /// * `sort` - Sort the results in the ascending/descending order of webhook creation
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
    ///         .webhooks
    ///         .get_webhooks(
    ///             &GetWebhooksQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_webhooks(
        &self,
        request: &GetWebhooksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "webhooks",
                None,
                QueryBuilder::new()
                    .serialize("type", request.r#type.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new webhook to receive real-time notifications for specified events.
    ///
    /// Use this to:
    /// - Set up event notifications for transactional or marketing emails
    /// - Configure webhook endpoints for campaign tracking
    /// - Enable real-time monitoring of email delivery status
    /// - Subscribe to contact list changes and updates
    /// - Implement custom event handling and automation
    ///
    /// Key information returned:
    /// - Created webhook ID and configuration
    /// - Success confirmation and setup details
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
    ///         .webhooks
    ///         .create_webhook(
    ///             &CreateWebhookRequest {
    ///                 url: "http://requestb.in/173lyyx1".to_string(),
    ///                 auth: None,
    ///                 batched: None,
    ///                 channel: None,
    ///                 description: None,
    ///                 domain: None,
    ///                 events: None,
    ///                 headers: None,
    ///                 r#type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_webhook(
        &self,
        request: &CreateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateWebhookResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note>
    /// This is an enterprise feature. Contact us to activate it for your account.
    /// </Note>
    ///
    /// Submits a request to export webhook event history as a CSV file. The download link is sent to the `notifyURL` you provide in the request body.
    ///
    /// Use this endpoint to:
    /// - Export webhook event history filtered by date range, event type, or email address
    /// - Generate reports for compliance, auditing, or performance analysis
    /// - Track delivery patterns and webhook reliability over time
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
    ///         .webhooks
    ///         .export_webhooks_history(
    ///             &ExportWebhooksHistoryRequest {
    ///                 event: ExportWebhooksHistoryRequestEvent::InvalidParameter,
    ///                 notify_url: "https://brevo.com".to_string(),
    ///                 r#type: ExportWebhooksHistoryRequestType::Transactional,
    ///                 days: None,
    ///                 email: None,
    ///                 end_date: None,
    ///                 message_id: None,
    ///                 sort: None,
    ///                 start_date: None,
    ///                 webhook_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_webhooks_history(
        &self,
        request: &ExportWebhooksHistoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExportWebhooksHistoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "webhooks/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves detailed information about a specific webhook configuration.
    ///
    /// Use this to:
    /// - Get complete webhook configuration and settings
    /// - Check webhook event subscriptions and triggers
    /// - Review authentication and security settings
    /// - Verify webhook URL and custom headers
    /// - Access webhook creation and modification history
    ///
    /// Key information returned:
    /// - Complete webhook details and configuration
    /// - Event types and channel subscriptions
    /// - Authentication credentials and methods
    /// - Custom headers and request settings
    /// - Webhook status and activity information
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - Id of the webhook
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
    ///     client.webhooks.get_webhook(1000000, None).await;
    /// }
    /// ```
    pub async fn get_webhook(
        &self,
        webhook_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("webhooks/{}", webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing webhook configuration and event subscriptions.
    ///
    /// Use this to:
    /// - Modify webhook event subscriptions and triggers
    /// - Update webhook URL and endpoint configuration
    /// - Change authentication settings and credentials
    /// - Adjust custom headers and request parameters
    /// - Enable or disable specific webhook events
    ///
    /// Key information returned:
    /// - Success confirmation of webhook updates
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - Id of the webhook
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
    ///         .webhooks
    ///         .update_webhook(
    ///             1000000,
    ///             &UpdateWebhookRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_webhook(
        &self,
        webhook_id: i64,
        request: &UpdateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("webhooks/{}", webhook_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a webhook and stops all event notifications.
    ///
    /// Use this to:
    /// - Remove unused or obsolete webhook configurations
    /// - Clean up webhook endpoints and subscriptions
    /// - Stop event notifications to specific URLs
    /// - Maintain organized webhook management
    ///
    /// Key information returned:
    /// - Success confirmation of webhook deletion
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - Id of the webhook
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
    ///     client.webhooks.delete_webhook(1000000, None).await;
    /// }
    /// ```
    pub async fn delete_webhook(
        &self,
        webhook_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("webhooks/{}", webhook_id),
                None,
                None,
                options,
            )
            .await
    }
}
