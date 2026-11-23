use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DomainsClient {
    pub http_client: HttpClient,
}

impl DomainsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves all domains associated with the account.
    ///
    /// Use this to:
    /// - List all domains
    /// - Verify domain existence
    /// - Check domain authentication and verification status
    /// - Monitor domain configuration and provider information
    /// - Review domain creation history and ownership
    ///
    /// Key information returned:
    /// - Domain details (ID, name, authentication status)
    /// - Verification and authentication states
    /// - Associated IP addresses and DNS providers
    /// - Creator information and creation timestamps
    /// - Pagination information for large domain lists
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
    ///     client.domains.get_domains(None).await;
    /// }
    /// ```
    pub async fn get_domains(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetDomainsResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "senders/domains", None, None, options)
            .await
    }

    /// Creates a new domain in Brevo.
    ///
    /// Use this to:
    /// - Add new domains for sending emails
    /// - Set up domain authentication for better deliverability
    /// - Configure DNS records for email authentication
    /// - Establish domain-based sender identities
    ///
    /// Key information returned:
    /// - Created domain ID and configuration
    /// - Required DNS records for authentication
    /// - Domain provider detection results
    /// - Setup instructions and next steps
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
    ///         .domains
    ///         .create_domain(
    ///             &CreateDomainRequest {
    ///                 name: "mycompany.com".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_domain(
        &self,
        request: &CreateDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateDomainResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "senders/domains",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves configuration of a specific domain, to know if the domain is valid or not.
    ///
    /// Use this to:
    /// - Check domain configuration
    /// - Validate a domain configuration
    /// - Monitor DNS record status
    /// - Troubleshoot authentication issues
    ///
    /// Key information returned:
    /// - Domain verification and authentication status
    /// - DNS records configuration and validation status
    /// - Detailed authentication requirements
    ///
    /// # Arguments
    ///
    /// * `domain_name` - Domain name
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
    ///         .domains
    ///         .get_domain_configuration(&"domainName".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_domain_configuration(
        &self,
        domain_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetDomainConfigurationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("senders/domains/{}", domain_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a domain from Brevo.
    ///
    /// Use this to:
    /// - Remove existing domains
    /// - Clean up unused domain configurations
    /// - Remove test domains
    ///
    /// Key information returned:
    /// - Success confirmation message
    ///
    /// # Arguments
    ///
    /// * `domain_name` - Domain name
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
    ///         .domains
    ///         .delete_domain(&"domainName".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_domain(
        &self,
        domain_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("senders/domains/{}", domain_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Authenticates a specific domain.
    ///
    /// Use this to:
    /// - Authenticate a domain
    /// - Verify DNS record configuration
    /// - Complete domain setup for sending
    /// - Enable domain for email authentication
    ///
    /// Key information returned:
    /// - Authentication success confirmation
    /// - Domain readiness status for email sending
    ///
    /// # Arguments
    ///
    /// * `domain_name` - Domain name
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
    ///         .domains
    ///         .authenticate_domain(&"domainName".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn authenticate_domain(
        &self,
        domain_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<AuthenticateDomainResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("senders/domains/{}/authenticate", domain_name),
                None,
                None,
                options,
            )
            .await
    }
}
