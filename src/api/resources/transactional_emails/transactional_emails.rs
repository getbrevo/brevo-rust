use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TransactionalEmailsClient {
    pub http_client: HttpClient,
}

impl TransactionalEmailsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of transactional contacts that have been blocked or unsubscribed, along with the reason for blocking (e.g. hard bounce, admin blocked, spam complaint, or unsubscription via email/API/Marketing Automation). Both `startDate` and `endDate` must be provided together when filtering by date range, and neither date can be in the future. Results default to 50 per page (max 100) and are sorted in descending order of record creation unless overridden with the `sort` parameter.
    ///
    /// # Arguments
    ///
    /// * `start_date` - **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) from which you want to fetch the blocked or unsubscribed contacts
    /// * `end_date` - **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) till which you want to fetch the blocked or unsubscribed contacts
    /// * `limit` - Number of documents returned per page
    /// * `offset` - Index of the first document on the page
    /// * `senders` - Comma separated list of emails of the senders from which contacts are blocked or unsubscribed
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
    ///         .transactional_emails
    ///         .get_transac_blocked_contacts(
    ///             &GetTransacBlockedContactsQueryRequest {
    ///                 start_date: None,
    ///                 end_date: None,
    ///                 limit: None,
    ///                 offset: None,
    ///                 senders: vec![],
    ///                 sort: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_transac_blocked_contacts(
        &self,
        request: &GetTransacBlockedContactsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTransacBlockedContactsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "smtp/blockedContacts",
                None,
                QueryBuilder::new()
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("senders", {
                        let joined = request
                            .senders
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Unblock or resubscribe a transactional contact by removing their email address from the blacklist. The email address must be URL-encoded in the path parameter and must be a valid email format. If the contact is not found in the blocklist, a 404 error is returned.
    ///
    /// # Arguments
    ///
    /// * `email` - contact email (urlencoded) to unblock.
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
    ///         .transactional_emails
    ///         .unblock_or_resubscribe_a_transactional_contact(&"email".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn unblock_or_resubscribe_a_transactional_contact(
        &self,
        email: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("smtp/blockedContacts/{}", email),
                None,
                None,
                options,
            )
            .await
    }

    /// Get the list of blocked domains
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
    ///     client.transactional_emails.get_blocked_domains(None).await;
    /// }
    /// ```
    pub async fn get_blocked_domains(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetBlockedDomainsResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "smtp/blockedDomains", None, None, options)
            .await
    }

    /// Blocks a new domain in order to avoid messages being sent to the same
    ///
    /// # Arguments
    ///
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
    ///         .transactional_emails
    ///         .block_new_domain(
    ///             &BlockNewDomainRequest {
    ///                 domain: "example.com".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn block_new_domain(
        &self,
        request: &BlockNewDomainRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "smtp/blockedDomains",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Unblocks an existing domain from the list of blocked domains
    ///
    /// # Arguments
    ///
    /// * `domain` - The name of the domain to be deleted
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
    ///         .transactional_emails
    ///         .delete_blocked_domain(&"domain".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_blocked_domain(
        &self,
        domain: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("smtp/blockedDomains/{}", domain),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete hardbounces. To use carefully (e.g. in case of temporary ISP failures)
    ///
    /// # Arguments
    ///
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
    ///         .transactional_emails
    ///         .delete_hardbounces(
    ///             &DeleteHardbouncesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_hardbounces(
        &self,
        request: &DeleteHardbouncesRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "smtp/deleteHardbounces",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send a transactional email to one or more recipients, either using inline HTML content or a pre-built template via `templateId`. You can schedule emails for future delivery using `scheduledAt` (UTC, up to 5-minute delay), send multiple personalized versions with `messageVersions` (max 2000 total recipients, 99 per version), and attach files via URL or base64-encoded content. A `sender` and `subject` are required when no `templateId` is provided; when a `templateId` is used, the template''s sender and subject are applied unless overridden.
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
    ///     client.transactional_emails.send_transac_email(&SendTransacEmailRequest {
    ///         html_content: Some("<html><head></head><body><p>Hello,</p>This is my first transactional email sent from Brevo.</p></body></html>".to_string()),
    ///         sender: Some(SendTransacEmailRequestSender {
    ///             email: Some("hello@brevo.com".to_string()),
    ///             name: Some("Alex from Brevo".to_string()),
    ///             ..Default::default()
    ///         }),
    ///         subject: Some("Hello from Brevo!".to_string()),
    ///         to: Some(vec![SendTransacEmailRequestToItem {
    ///             email: "johndoe@example.com".to_string(),
    ///             name: Some("John Doe".to_string()),
    ///             ..Default::default()
    ///         }]),
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn send_transac_email(
        &self,
        request: &SendTransacEmailRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendTransacEmailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "smtp/email",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete scheduled batch of emails by batchId or single scheduled email by messageId
    ///
    /// # Arguments
    ///
    /// * `identifier` - The `batchId` of scheduled emails batch (must be a valid UUIDv4) or the `messageId` of scheduled email (enclosed in angle brackets with @ sign, e.g. `<...@domain>`).
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
    ///         .transactional_emails
    ///         .delete_scheduled_email_by_id(&"4320f270-a4e3-4a2e-b591-edfe30a5e627".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_scheduled_email_by_id(
        &self,
        identifier: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("smtp/email/{}", identifier),
                None,
                None,
                options,
            )
            .await
    }

    /// Fetch scheduled batch of emails by batchId or single scheduled email by messageId (Can retrieve data upto 30 days old)
    ///
    /// # Arguments
    ///
    /// * `identifier` - The `batchId` of scheduled emails batch (must be a valid UUIDv4) or the `messageId` of scheduled email (enclosed in angle brackets with @ sign, e.g. `<...@domain>`). When using `messageId`, the `limit`, `offset`, `sort`, and `status` query parameters are ignored.
    /// * `start_date` - Mandatory if `endDate` is used. Starting date (YYYY-MM-DD) from which you want to fetch the list. Cannot be more than 30 days older than the current date.
    /// * `end_date` - Mandatory if `startDate` is used. Ending date (YYYY-MM-DD) till which you want to fetch the list. Maximum time period that can be selected is one month.
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed. Not valid when identifier is `messageId`.
    /// * `status` - Filter the records by `status` of the scheduled email batch or message. Not valid when identifier is `messageId`.
    /// * `limit` - Number of documents returned per page. Not valid when identifier is `messageId`.
    /// * `offset` - Index of the first document on the page.  Not valid when identifier is `messageId`.
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
    ///         .transactional_emails
    ///         .get_scheduled_email_by_id(
    ///             &"4320f270-a4e3-4a2e-b591-edfe30a5e627".to_string(),
    ///             &GetScheduledEmailByIdQueryRequest {
    ///                 start_date: Some(NaiveDate::parse_from_str("2022-02-02", "%Y-%m-%d").unwrap()),
    ///                 end_date: Some(NaiveDate::parse_from_str("2022-03-02", "%Y-%m-%d").unwrap()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_scheduled_email_by_id(
        &self,
        identifier: &str,
        request: &GetScheduledEmailByIdQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetScheduledEmailByIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("smtp/emailStatus/{}", identifier),
                None,
                QueryBuilder::new()
                    .date("startDate", request.start_date.clone())
                    .date("endDate", request.end_date.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("status", request.status.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This endpoint will show the list of emails for past 30 days by default. To retrieve emails before that time, please pass startDate and endDate in query filters.
    ///
    /// # Arguments
    ///
    /// * `email` - **Mandatory if templateId and messageId are not passed in query filters.** Email address to which transactional email has been sent.
    /// * `template_id` - **Mandatory if email and messageId are not passed in query filters.** Id of the template that was used to compose transactional email.
    /// * `message_id` - **Mandatory if templateId and email are not passed in query filters.** Message ID of the transactional email sent.
    /// * `start_date` - **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) from which you want to fetch the list. **Maximum time period that can be selected is one month**.
    /// * `end_date` - **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) till which you want to fetch the list. **Maximum time period that can be selected is one month.**
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    /// * `limit` - Number of documents returned per page
    /// * `offset` - Index of the first document in the page
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
    ///         .transactional_emails
    ///         .get_transac_emails_list(
    ///             &GetTransacEmailsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_transac_emails_list(
        &self,
        request: &GetTransacEmailsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTransacEmailsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "smtp/emails",
                None,
                QueryBuilder::new()
                    .string("email", request.email.clone())
                    .int("templateId", request.template_id.clone())
                    .string("messageId", request.message_id.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .serialize("sort", request.sort.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Note title="How to get uuid">You can get the uuid using either of the following methods:
    /// Send a GET request to https://api.brevo.com/v3/smtp/emails and pass the message_id in the url. Use your api-key to authenticate the request and you will get your uuid as a response.
    /// The uuid can also be fetched from the transactional logs page in your Brevo account, from the address URL.</Note>
    ///
    /// # Arguments
    ///
    /// * `uuid` - Unique id of the transactional email that has been sent to a particular contact
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
    ///         .transactional_emails
    ///         .get_transac_email_content(&"uuid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_transac_email_content(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetTransacEmailContentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("smtp/emails/{}", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete SMTP transactional log entries identified by a message ID (enclosed in angle brackets with an @ sign) or a valid email address. Optionally narrow the deletion to a specific date range using `from_date` and `to_date` query parameters (YYYY-MM-DD format). The operation also removes any associated stored email preview content.
    ///
    /// # Arguments
    ///
    /// * `identifier` - MessageId or email address of the transactional log(s) to delete. Must be a valid message ID (enclosed in angle brackets with @ sign) or a valid email address.
    /// * `from_date` - Starting date (YYYY-MM-DD) to narrow down logs for deletion
    /// * `to_date` - Ending date (YYYY-MM-DD) to narrow down logs for deletion
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
    ///         .transactional_emails
    ///         .delete_an_smtp_transactional_log(
    ///             &"identifier".to_string(),
    ///             &DeleteAnSmtpTransactionalLogQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_an_smtp_transactional_log(
        &self,
        identifier: &str,
        request: &DeleteAnSmtpTransactionalLogQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("smtp/log/{}", identifier),
                None,
                QueryBuilder::new()
                    .string("from_date", request.from_date.clone())
                    .string("to_date", request.to_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This endpoint will show the aggregated stats for past 90 days by default if `startDate` and `endDate` OR `days` is not passed. The date range can not exceed 90 days
    ///
    /// # Arguments
    ///
    /// * `start_date` - **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    /// * `end_date` - **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    /// * `days` - Number of days in the past including today (positive integer, maximum 90). _Not compatible with 'startDate' and 'endDate'_. Defaults to 90 if neither dates nor days are provided.
    /// * `tag` - Tag of the emails
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
    ///         .transactional_emails
    ///         .get_aggregated_smtp_report(
    ///             &GetAggregatedSmtpReportQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_aggregated_smtp_report(
        &self,
        request: &GetAggregatedSmtpReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAggregatedSmtpReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "smtp/statistics/aggregatedReport",
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

    /// This endpoint will show the aggregated stats for past 30 days by default if `startDate` and `endDate` OR `days` is not passed. The date range can not exceed 90 days
    ///
    /// # Arguments
    ///
    /// * `limit` - Number limitation for the result returned
    /// * `offset` - Beginning point in the list to retrieve from.
    /// * `start_date` - **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    /// * `end_date` - **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    /// * `days` - Number of days in the past including today (positive integer, maximum 90). _Not compatible with 'startDate' and 'endDate'_. Defaults to 30 if neither dates nor days are provided.
    /// * `email` - Filter the report for a specific email addresses
    /// * `event` - Filter the report for a specific event type
    /// * `tags` - Filter the report for tags (serialized and urlencoded array)
    /// * `message_id` - Filter on a specific message id
    /// * `template_id` - Filter on a specific template id
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
    ///         .transactional_emails
    ///         .get_email_event_report(
    ///             &GetEmailEventReportQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_email_event_report(
        &self,
        request: &GetEmailEventReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetEmailEventReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "smtp/statistics/events",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("days", request.days.clone())
                    .string("email", request.email.clone())
                    .serialize("event", request.event.clone())
                    .string("tags", request.tags.clone())
                    .string("messageId", request.message_id.clone())
                    .int("templateId", request.template_id.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This endpoint will show the aggregated stats per day for the past 10 days by default if `startDate` and `endDate` OR `days` is not passed. The date range can not exceed 30 days.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents returned per page
    /// * `offset` - Index of the first document on the page
    /// * `start_date` - **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD)
    /// * `end_date` - **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD)
    /// * `days` - Number of days in the past including today (positive integer, maximum 30). _Not compatible with 'startDate' and 'endDate'_
    /// * `tag` - Tag of the emails
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
    ///         .transactional_emails
    ///         .get_smtp_report(
    ///             &GetSmtpReportQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_smtp_report(
        &self,
        request: &GetSmtpReportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSmtpReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "smtp/statistics/reports",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
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

    /// Generate a fully rendered preview of a transactional email template by resolving dynamic variables. Provide either an `email` address (to populate variables from the contact''s attributes) or a `params` object with key-value pairs for manual substitution; at least one of these is required alongside the mandatory `templateId`. The response includes the rendered HTML, subject, sender details, preview text, and any feed names used in the template.
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
    ///         .transactional_emails
    ///         .post_preview_smtp_email_templates(
    ///             &PostPreviewSmtpEmailTemplatesRequest::Unknown(serde_json::json!({"key":"value"})),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn post_preview_smtp_email_templates(
        &self,
        request: &PostPreviewSmtpEmailTemplatesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostPreviewSmtpEmailTemplatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "smtp/template/preview",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a paginated list of all transactional email templates (including automation templates) with their details such as name, subject, sender, status, HTML content, and timestamps. Results default to 50 per page (max 1000) and are sorted in descending creation order unless overridden. You can filter by active/inactive status using `templateStatus` and by editor type using `editorType` (currently only `richTextEditor` is supported).
    ///
    /// # Arguments
    ///
    /// * `template_status` - Filter on the status of the template. Active = true, inactive = false
    /// * `limit` - Number of documents returned per page
    /// * `offset` - Index of the first document in the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    /// * `editor_type` - Filter on the editor type used to create the template. Currently only `richTextEditor` is supported as a filter value.
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
    ///         .transactional_emails
    ///         .get_smtp_templates(
    ///             &GetSmtpTemplatesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_smtp_templates(
        &self,
        request: &GetSmtpTemplatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSmtpTemplatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "smtp/templates",
                None,
                QueryBuilder::new()
                    .bool("templateStatus", request.template_status.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .string("editorType", request.editor_type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new transactional email template with the specified sender, subject, and content. The `sender`, `subject`, and `templateName` fields are required. Template content can be provided via `htmlContent` (minimum 10 characters) or `htmlUrl`; at least one must be supplied. Templates are created as inactive by default unless `isActive` is explicitly set to `true`.
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
    ///         .transactional_emails
    ///         .create_smtp_template(
    ///             &CreateSmtpTemplateRequest {
    ///                 sender: CreateSmtpTemplateRequestSender {
    ///                     ..Default::default()
    ///                 },
    ///                 subject: "Thanks for your purchase !".to_string(),
    ///                 template_name: "Order Confirmation - EN".to_string(),
    ///                 attachment_url: None,
    ///                 html_content: None,
    ///                 html_url: None,
    ///                 is_active: None,
    ///                 reply_to: None,
    ///                 tag: None,
    ///                 to_field: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_smtp_template(
        &self,
        request: &CreateSmtpTemplateRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateSmtpTemplateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "smtp/templates",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the full details of a specific transactional email template by its numeric ID or custom template identifier string. The response includes the template name, subject, sender information, HTML content, active status, creation and modification timestamps, reply-to address, tag, and a `doiTemplate` flag indicating whether the template is a double opt-in template (detected by the presence of optin-related tags or variables in the content).
    ///
    /// # Arguments
    ///
    /// * `template_id` - ID of the template. Can be a numeric template ID or a custom template identifier string (alphanumeric, hyphens, and underscores only, max 64 characters, must start with a letter).
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
    ///         .transactional_emails
    ///         .get_smtp_template(&GetSmtpTemplateRequestTemplateId::Long(1000000), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_smtp_template(
        &self,
        template_id: &GetSmtpTemplateRequestTemplateId,
        options: Option<RequestOptions>,
    ) -> Result<GetSmtpTemplateOverview, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("smtp/templates/{}", template_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing transactional email template by its numeric ID or custom template identifier string. All fields in the request body are optional; only the provided fields will be updated. You can update the template name, subject, sender, reply-to address, HTML content (via `htmlContent` or `htmlUrl`), active status, tag, attachment URL, and the personalized `toField`. Only one of sender email or sender ID should be provided per request.
    ///
    /// # Arguments
    ///
    /// * `template_id` - ID of the template. Can be a numeric template ID or a custom template identifier string.
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
    ///         .transactional_emails
    ///         .update_smtp_template(
    ///             &UpdateSmtpTemplateRequestTemplateId::Long(1000000),
    ///             &UpdateSmtpTemplateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_smtp_template(
        &self,
        template_id: &UpdateSmtpTemplateRequestTemplateId,
        request: &UpdateSmtpTemplateRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("smtp/templates/{}", template_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Permanently delete a transactional email template by its numeric ID. Only inactive templates can be deleted; attempting to delete an active template returns a 405 error. To deactivate a template before deletion, use `PUT /smtp/templates/{templateId}` with `isActive` set to `false`. Deletion also removes associated newsletter template data and triggers asynchronous cleanup of shared assets.
    ///
    /// # Arguments
    ///
    /// * `template_id` - id of the template
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
    ///         .transactional_emails
    ///         .delete_smtp_template(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_smtp_template(
        &self,
        template_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("smtp/templates/{}", template_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Send a test email of the specified transactional template to one or more recipients. Provide an array of email addresses in the `emailTo` field; if left empty, the test mail is sent to your entire test list. You can send a maximum of 50 test emails per day, and all provided email addresses must be valid.
    ///
    /// # Arguments
    ///
    /// * `template_id` - Id of the template
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
    ///         .transactional_emails
    ///         .send_test_template(
    ///             1000000,
    ///             &SendTestEmail {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_test_template(
        &self,
        template_id: i64,
        request: &SendTestEmail,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("smtp/templates/{}/sendTest", template_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
