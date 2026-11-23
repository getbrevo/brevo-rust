use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TransactionalWhatsAppClient {
    pub http_client: HttpClient,
}

impl TransactionalWhatsAppClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// <Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
    /// [Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
    /// This endpoint is used to send a WhatsApp message. <br/>(**The first message you send using the API must contain a Template ID. You must create a template on WhatsApp on the Brevo platform to fetch the Template ID.**)
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
    ///         .transactional_whats_app
    ///         .send_whatsapp_message(
    ///             &SendWhatsappMessageRequest::SendWhatsappMessageRequestParams(
    ///                 SendWhatsappMessageRequestParams {
    ///                     contact_numbers: vec!["contactNumbers".to_string()],
    ///                     sender_number: "senderNumber".to_string(),
    ///                     template_id: 123,
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_whatsapp_message(
        &self,
        request: &SendWhatsappMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendWhatsappMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "whatsapp/sendMessage",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint will show the unaggregated statistics for WhatsApp activity (30 days by default if `startDate` and `endDate` or `days` is not passed. The date range can not exceed 90 days)
    ///
    /// # Arguments
    ///
    /// * `limit` - Number limitation for the result returned
    /// * `offset` - Beginning point in the list to retrieve from
    /// * `start_date` - **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    /// * `end_date` - **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    /// * `days` - Number of days in the past including today (positive integer). _Not compatible with 'startDate' and 'endDate'_
    /// * `contact_number` - Filter results for specific contact (WhatsApp Number with country code. Example, 85264318721)
    /// * `event` - Filter the report for a specific event type
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
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
    ///         .transactional_whats_app
    ///         .get_whatsapp_event_report(
    ///             &GetWhatsappEventReportQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_whatsapp_event_report(
        &self,
        request: &GetWhatsappEventReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetWhatsappEventReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "whatsapp/statistics/events",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("days", request.days.clone())
                    .string("contactNumber", request.contact_number.clone())
                    .serialize("event", request.event.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }
}
