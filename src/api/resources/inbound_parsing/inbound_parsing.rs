use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InboundParsingClient {
    pub http_client: HttpClient,
}

impl InboundParsingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// This endpoint will show the list of all the events for the received emails. When no date range is provided, the last 30 days of events are returned by default.
    ///
    /// # Arguments
    ///
    /// * `sender` - Email address of the sender.
    /// * `start_date` - Mandatory if endDate is used. Starting date (YYYY-MM-DD or YYYY-MM-DDTHH:mm:ss.SSSZ) from which you want to fetch the list. Maximum time period that can be selected is 30 days. Must not be in the future.
    /// * `end_date` - Mandatory if startDate is used. Ending date (YYYY-MM-DD or YYYY-MM-DDTHH:mm:ss.SSSZ) till which you want to fetch the list. Maximum time period that can be selected is 30 days. Must not be in the future.
    /// * `limit` - Number of documents returned per page
    /// * `offset` - Index of the first document on the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation
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
    ///         .inbound_parsing
    ///         .get_inbound_email_events(
    ///             &GetInboundEmailEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_inbound_email_events(
        &self,
        request: &GetInboundEmailEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetInboundEmailEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "inbound/events",
                None,
                QueryBuilder::new()
                    .string("sender", request.sender.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This endpoint will show the list of all events history for one particular received email.
    ///
    /// # Arguments
    ///
    /// * `uuid` - UUID to fetch events specific to received email
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
    ///         .inbound_parsing
    ///         .get_inbound_email_events_by_uuid(&"uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_inbound_email_events_by_uuid(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetInboundEmailEventsByUuidResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("inbound/events/{}", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// This endpoint will retrieve inbound attachment with download token.
    ///
    /// # Arguments
    ///
    /// * `download_token` - Token to fetch a particular attachment
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .inbound_parsing
    ///         .get_inbound_email_attachment(&"downloadToken".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_inbound_email_attachment(
        &self,
        download_token: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("inbound/attachments/{}", download_token),
                None,
                None,
                options,
            )
            .await
    }
}
