use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TransactionalSmsClient {
    pub http_client: HttpClient,
}

impl TransactionalSmsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// <Note>If the user includes stop code in the Transactional SMS, then it will be switched to Marketing SMS automatically and it will be interpreted as a Marketing SMS. To send Transactional SMS as Transactional, it is important not to use stop code.
    /// Note: For adding a stop code, client has to add reply STOP to [STOP_CODE] and the [STOP_CODE] will be replaced with the number.</Note>
    /// <Note title="For end users in France">Transactional SMS can be sent at any time without time restrictions. However, if a message is categorized as Marketing, it must adhere to specific time restrictions. Messages sent outside of these restricted hours will experience delays and will be processed during allowable times. Specifically, Marketing SMS cannot be processed between 10pm and 8am, on Sundays, and on French public holidays.</Note>
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
    ///         .transactional_sms
    ///         .send_async_transactional_sms(
    ///             &SendTransacSms {
    ///                 recipient: "33689965433".to_string(),
    ///                 sender: "MyShop".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_async_transactional_sms(
        &self,
        request: &SendTransacSms,
        options: Option<RequestOptions>,
    ) -> Result<SendAsyncTransactionalSmsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "transactionalSMS/send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send a transactional SMS message to a single mobile number. The `sender`, `recipient`, and either `content` or `templateId` fields are required. The sender name is limited to 11 alphanumeric characters or 15 numeric characters, and the recipient must be a valid international phone number (6-15 digits, optional leading +). Tags can be a string or an array of up to 10 strings. The SMS type defaults to `transactional` but can be set to `marketing`; if the content includes a stop code, it is automatically treated as marketing. Returns the message ID, SMS count, credits used, and remaining credits.
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
    ///         .transactional_sms
    ///         .send_transac_sms(
    ///             &SendTransacSms {
    ///                 recipient: "33689965433".to_string(),
    ///                 sender: "MyShop".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_transac_sms(
        &self,
        request: &SendTransacSms,
        options: Option<RequestOptions>,
    ) -> Result<SendTransacSmsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "transactionalSMS/sms",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve an aggregated report of your transactional SMS activity over a specified time period, including counts for requests, delivered, hard bounces, soft bounces, blocked, unsubscribed, replied, accepted, rejected, and skipped messages. Filter by date range using `startDate` and `endDate` (both required together, YYYY-MM-DD format) or by a number of past `days` (not compatible with date range). You can further narrow results by `tag`. If no date filter is provided, the report covers all available data and returns the auto-detected date range.
    ///
    /// # Arguments
    ///
    /// * `start_date` - **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    /// * `end_date` - **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    /// * `days` - Number of days in the past including today (positive integer). **Not compatible with startDate and endDate**
    /// * `tag` - Filter on a tag
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
    ///         .transactional_sms
    ///         .get_transac_aggregated_sms_report(
    ///             &GetTransacAggregatedSmsReportQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_transac_aggregated_sms_report(
        &self,
        request: &GetTransacAggregatedSmsReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTransacAggregatedSmsReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "transactionalSMS/statistics/aggregatedReport",
                None,
                QueryBuilder::new()
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("days", request.days.clone())
                    .string("tag", request.tag.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a paginated list of individual SMS event records (unaggregated), including event type, phone number, message ID, timestamp, tag, and reason or reply content where applicable. Results default to 50 per page (max 100) and are sorted in descending order unless overridden. Filter by date range (`startDate`/`endDate`), past `days` (not compatible with date range), specific `event` type (e.g. delivered, bounces, replies), `phoneNumber`, or `tags`. Bounce events include the failure reason, and reply events include the reply content.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `start_date` - **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    /// * `end_date` - **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    /// * `offset` - Index of the first document of the page
    /// * `days` - Number of days in the past including today (positive integer). **Not compatible with 'startDate' and 'endDate'**
    /// * `phone_number` - Filter the report for a specific phone number
    /// * `event` - Filter the report for specific events
    /// * `tags` - Filter the report for specific tags passed as a serialized urlencoded array
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
    ///         .transactional_sms
    ///         .get_sms_events(
    ///             &GetSmsEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_sms_events(
        &self,
        request: &GetSmsEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSmsEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "transactionalSMS/statistics/events",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("offset", request.offset.clone())
                    .int("days", request.days.clone())
                    .string("phoneNumber", request.phone_number.clone())
                    .serialize("event", request.event.clone())
                    .string("tags", request.tags.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a day-by-day breakdown of your transactional SMS activity, with each entry containing the date and counts for requests, delivered, hard bounces, soft bounces, blocked, unsubscribed, replied, accepted, rejected, and skipped messages. Filter by date range using `startDate` and `endDate` (both required together, YYYY-MM-DD format), by a number of past `days` (not compatible with date range), or by `tag`. Results are sorted in descending order by default unless overridden with the `sort` parameter.
    ///
    /// # Arguments
    ///
    /// * `start_date` - **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    /// * `end_date` - **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    /// * `days` - Number of days in the past including today (positive integer). **Not compatible with 'startDate' and 'endDate'**
    /// * `tag` - Filter on a tag
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
    ///         .transactional_sms
    ///         .get_transac_sms_report(
    ///             &GetTransacSmsReportQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_transac_sms_report(
        &self,
        request: &GetTransacSmsReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTransacSmsReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "transactionalSMS/statistics/reports",
                None,
                QueryBuilder::new()
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("days", request.days.clone())
                    .string("tag", request.tag.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }
}
