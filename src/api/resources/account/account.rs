use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AccountClient {
    pub http_client: HttpClient,
}

impl AccountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves details of your Brevo account.
    ///
    /// **Use this to:**
    /// - Get account information (email, name, company, address)
    /// - Check plan details (type, credits, expiration)
    /// - Get relay information (for transactional emails)
    /// - Check Marketing Automation status
    /// - Access organization and user identifiers
    ///
    /// **Key information returned:**
    /// - Complete account details (organization ID, user ID, company information)
    /// - Address and contact information
    /// - Plan configurations and credit allocations across different verticals
    /// - Marketing Automation settings and tracker key (when enabled)
    /// - SMTP relay configuration for transactional emails
    /// - Enterprise features availability status
    ///
    /// **Important considerations:**
    /// - Provides comprehensive account overview for billing and configuration management
    /// - Essential for understanding current plan limitations and feature availability
    /// - Marketing Automation key is only returned when Marketing Automation is enabled on the account
    /// - Plan verticals show detailed breakdown across Marketing, Chat, and CRM categories (only returned when plan verticals are available)
    /// - Relay configuration crucial for transactional email setup and deliverability
    /// - Enterprise status determines access to advanced features and sub-account management
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
    ///     client.account.get_account(None).await;
    /// }
    /// ```
    pub async fn get_account(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetAccountResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "account", None, None, options)
            .await
    }

    /// Retrieves user activity logs from your organization for security monitoring and audit compliance.
    ///
    /// Use this to:
    /// - Monitor user login activities and access patterns
    /// - Track account modifications and configuration changes
    /// - Generate security audit reports and compliance documentation
    /// - Investigate suspicious activities and unauthorized access
    /// - Monitor team member actions and account usage
    ///
    /// Key information returned:
    /// - Complete user activity details and timestamps
    /// - User identification (email, IP address, browser)
    /// - Action types and activity descriptions
    /// - Security-relevant events and access logs
    /// - Historical activity data for audit trails
    ///
    /// Note: Requires Enterprise plan for access to organization activity logs.
    ///
    /// # Arguments
    ///
    /// * `start_date` - Mandatory if endDate is used. Enter start date in UTC date (YYYY-MM-DD)
    /// format to filter the activity in your account. Maximum time period that
    /// can be selected is one month. Additionally, you can retrieve activity
    /// logs from the past 12 months from the date of your search.
    /// * `end_date` - Mandatory if startDate is used. Enter end date in UTC date (YYYY-MM-DD)
    /// format to filter the activity in your account. Maximum time period that
    /// can be selected is one month.
    /// * `email` - Enter the user's email address to filter their activity in the account.
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page.
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
    ///         .account
    ///         .get_account_activity(
    ///             &GetAccountActivityQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_account_activity(
        &self,
        request: &GetAccountActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAccountActivityResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "organization/activities",
                None,
                QueryBuilder::new()
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .string("email", request.email.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }
}
