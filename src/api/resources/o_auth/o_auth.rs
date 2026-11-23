use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OAuthClient {
    pub http_client: HttpClient,
}

impl OAuthClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Exchanges an app's client_id/client_secret for a short-lived access token using the OAuth 2.0 client_credentials grant (RFC 6749 §4.4). Tokens are valid for about one hour; there is no refresh token.
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
    ///         .o_auth
    ///         .get_o_auth_m2m_token(
    ///             &GetOAuthM2MTokenRequest {
    ///                 grant_type: "client_credentials".to_string(),
    ///                 client_id: "client_id".to_string(),
    ///                 client_secret: "client_secret".to_string(),
    ///                 scope: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_o_auth_m2m_token(
        &self,
        request: &GetOAuthM2MTokenRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetOAuthM2MTokenResponse, ApiError> {
        self.http_client
            .execute_form_request(
                Method::POST,
                "../realms/partner/oauth/token",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
