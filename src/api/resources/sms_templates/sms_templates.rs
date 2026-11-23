use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SmsTemplatesClient {
    pub http_client: HttpClient,
}

impl SmsTemplatesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

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
    ///         .sms_templates
    ///         .get_sms_templates(
    ///             &GetSmsTemplatesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_sms_templates(
        &self,
        request: &GetSmsTemplatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSmsTemplatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "transactionalSMS/templates",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }
}
