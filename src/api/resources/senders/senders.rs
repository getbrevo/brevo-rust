use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SendersClient {
    pub http_client: HttpClient,
}

impl SendersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves a list of all email senders from your Brevo account with optional filtering.
    ///
    /// **Use this to:**
    /// - Get all available senders for email campaign setup
    /// - Find sender details including ID, name, and email address
    /// - Filter senders by IP address for dedicated IP users
    /// - Filter senders by domain for domain-specific configurations
    /// - Monitor sender configuration and status
    ///
    /// **Key information returned:**
    /// - Sender details (ID, name, email address)
    /// - Sender status and verification information
    /// - Associated IP addresses and domains (for dedicated IP accounts)
    /// - Sender configuration settings
    ///
    /// **Important considerations:**
    /// - Standard accounts show empty IP arrays, dedicated IP accounts show IP assignments
    /// - Filtering by IP only available for accounts with dedicated IPs
    /// - Domain filtering helps organize senders by business units or brands
    /// - Sender status indicates if sender is active and ready for campaign use
    /// - Email verification required before sender can be used in campaigns
    ///
    /// # Arguments
    ///
    /// * `ip` - Filter your senders for a specific ip. **Available for dedicated IP usage only**
    /// * `domain` - Filter your senders for a specific domain
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
    ///         .senders
    ///         .get_senders(
    ///             &GetSendersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_senders(
        &self,
        request: &GetSendersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSendersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "senders",
                None,
                QueryBuilder::new()
                    .string("ip", request.ip.clone())
                    .string("domain", request.domain.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new email sender in your Brevo account.
    ///
    /// **Use this to:**
    /// - Add new senders for email campaigns
    /// - Configure sender identity (name and email)
    /// - Associate dedicated IPs with the sender (for dedicated IP accounts)
    /// - Set up domain-based sender configurations
    ///
    /// **Key information returned:**
    /// - Created sender ID
    /// - DKIM and SPF configuration status
    /// - Success confirmation
    ///
    /// **Important considerations:**
    /// - Verification email sent to specified sender address
    /// - DKIM and SPF configuration affects deliverability
    /// - Dedicated IP accounts require IP association during creation
    /// - IP weights must sum to 100 when specified
    /// - Sender must be verified before use in campaigns
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
    ///         .senders
    ///         .create_sender(
    ///             &CreateSenderRequest {
    ///                 email: "support@example.com".to_string(),
    ///                 name: "Support Team".to_string(),
    ///                 ips: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_sender(
        &self,
        request: &CreateSenderRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateSenderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "senders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves all dedicated IPs associated with your Brevo account.
    ///
    /// Use this to:
    /// - List all your dedicated IPs
    /// - Check the status of your dedicated IPs (active/inactive)
    /// - Find IP addresses and associated domains for configuration purposes
    /// - Monitor your IP reputation and deliverability
    /// - Verify available IPs for sender configuration
    ///
    /// Key information returned:
    /// - IP ID and address
    /// - Associated domain
    /// - Active status
    /// - IP configuration details
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
    ///     client.senders.get_ips(None).await;
    /// }
    /// ```
    pub async fn get_ips(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetIpsResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "senders/ips", None, None, options)
            .await
    }

    /// Updates an existing email sender's configuration.
    ///
    /// Use this to:
    /// - Modify sender display name or email address
    /// - Update dedicated IP associations
    /// - Change sender configuration settings
    /// - Correct sender information
    ///
    /// Key information returned:
    /// - Success confirmation
    /// - Updated sender details
    ///
    /// # Arguments
    ///
    /// * `sender_id` - Id of the sender
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
    ///         .senders
    ///         .update_sender(
    ///             1000000,
    ///             &UpdateSenderRequest {
    ///                 name: Some("New Support Team".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_sender(
        &self,
        sender_id: i64,
        request: &UpdateSenderRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("senders/{}", sender_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an email sender from your Brevo account.
    ///
    /// Use this to:
    /// - Remove senders that are no longer needed
    /// - Clean up sender configurations
    /// - Remove duplicate or test senders
    ///
    /// Key information returned:
    /// - Success confirmation message
    ///
    /// # Arguments
    ///
    /// * `sender_id` - Id of the sender
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
    ///     client.senders.delete_sender(1000000, None).await;
    /// }
    /// ```
    pub async fn delete_sender(
        &self,
        sender_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("senders/{}", sender_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves the dedicated IPs associated with a specific sender.
    ///
    /// Use this to:
    /// - Check IP configuration for a sender
    /// - Verify dedicated IP associations
    /// - Get IP details for troubleshooting
    /// - Monitor sender IP configuration
    ///
    /// Key information returned:
    /// - List of associated dedicated IPs
    /// - IP addresses and domain configurations
    /// - IP status and settings
    ///
    /// # Arguments
    ///
    /// * `sender_id` - Id of the sender
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
    ///     client.senders.get_ips_from_sender(1000000, None).await;
    /// }
    /// ```
    pub async fn get_ips_from_sender(
        &self,
        sender_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetIpsFromSenderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("senders/{}/ips", sender_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Validates a sender using the OTP (One-Time Password) received via email.
    ///
    /// Use this to:
    /// - Complete sender verification process
    /// - Activate a newly created sender
    /// - Verify ownership of the sender email address
    /// - Enable the sender for use in email campaigns
    ///
    /// Key information returned:
    /// - Success confirmation of sender verification
    /// - Sender activation status
    ///
    /// # Arguments
    ///
    /// * `sender_id` - Id of the sender
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
    ///         .senders
    ///         .validate_sender_by_otp(1000000, &ValidateSenderByOtpRequest { otp: 123456 }, None)
    ///         .await;
    /// }
    /// ```
    pub async fn validate_sender_by_otp(
        &self,
        sender_id: i64,
        request: &ValidateSenderByOtpRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("senders/{}/validate", sender_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
