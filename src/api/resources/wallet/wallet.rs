use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WalletClient {
    pub http_client: HttpClient,
}

impl WalletClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generate a wallet installation URL for a specific contact. The returned URL points to the pass installation page and encodes the pass, contact and organization identifiers as an encrypted token, so it can be shared with the contact (email, SMS, QR code, ...) to add the pass to their Apple Wallet or Google Wallet.
    ///
    /// # Arguments
    ///
    /// * `pass_id` - Pass ID. The unique identifier of the wallet pass for which to generate an installation URL.
    /// * `contact_id` - The Brevo contact ID the installation URL is generated for.
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
    ///         .wallet
    ///         .get_wallet_pass_install_url(&"passId".to_string(), 1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_wallet_pass_install_url(
        &self,
        pass_id: &str,
        contact_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<WalletPassInstallUrl, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("wallet/passes/{}/installUrl/{}", pass_id, contact_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Record that a wallet pass installation was scanned or redeemed. This issues a `wallet_pass_scanned` event that can trigger automations and update contact segments (for example to track store visits, event attendance or loyalty check-ins). The request body is optional; any details supplied (location, amount spent, event tag, custom data) are carried on the event.
    ///
    /// # Arguments
    ///
    /// * `installation_id` - Installation ID. The unique identifier of the pass installation being redeemed (as issued when the contact added the pass to their wallet).
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
    ///         .wallet
    ///         .redeem_wallet_pass(
    ///             &"installationId".to_string(),
    ///             &WalletPassRedeemRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn redeem_wallet_pass(
        &self,
        installation_id: &str,
        request: &WalletPassRedeemRequest,
        options: Option<RequestOptions>,
    ) -> Result<WalletPassRedeemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("wallet/passes/{}/redeem", installation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
