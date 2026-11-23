use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PaymentsClient {
    pub http_client: HttpClient,
}

impl PaymentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a new payment request for a Brevo contact. The request requires a reference (displayed on the payment page), a contact ID, and a cart with currency and amount in cents. You can optionally configure a custom success redirect URL and enable email notifications with reminders. Returns the payment request ID and its public payment URL. A `403` error is returned if Brevo Payments is not activated or the account is not validated.
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
    ///         .payments
    ///         .create_payment_request(
    ///             &CreatePaymentRequestRequest {
    ///                 cart: Cart {
    ///                     currency: "EUR".to_string(),
    ///                     specific_amount: 1200,
    ///                 },
    ///                 contact_id: 43,
    ///                 reference: "Invoice #INV0001".to_string(),
    ///                 configuration: None,
    ///                 description: None,
    ///                 notification: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_payment_request(
        &self,
        request: &CreatePaymentRequestRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePaymentRequestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "payments/requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the details of a specific payment request by its ID. The response includes the reference, status (created, sent, reminderSent, or paid), cart details, notification configuration, contact ID, and the number of reminders sent. Returns a `404` error if no payment request matches the provided ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the payment Request
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
    ///         .payments
    ///         .get_payment_request(&"050db7b0-9bb7-4c1e-9c68-5a8dace8c1dc".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_payment_request(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPaymentRequestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("payments/requests/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a payment request by its UUID. Once deleted, the payment request can no longer be accessed or paid. Returns a `404` error if no payment request matches the provided ID, and a `403` error if Brevo Payments is not activated or the account is not validated.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the payment request.
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
    ///         .payments
    ///         .delete_payment_request(&"9ae7d68a-565c-4695-9381-d8fb3e3a14cc".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_payment_request(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("payments/requests/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
