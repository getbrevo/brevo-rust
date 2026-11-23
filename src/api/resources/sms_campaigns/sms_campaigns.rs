use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SmsCampaignsClient {
    pub http_client: HttpClient,
}

impl SmsCampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of all your SMS campaigns with their statistics and recipient information. Results can be filtered by status and date range, with a default limit of 500 and maximum of 1000 per page. The sort order defaults to descending by creation date; date filters are only available when status is not passed or is set to sent.
    ///
    /// # Arguments
    ///
    /// * `status` - Status of campaign.
    /// * `start_date` - **Mandatory if endDate is used.** Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent SMS campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. `startDate` must not be in the future.
    /// * `end_date` - **Mandatory if startDate is used.** Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent SMS campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. `endDate` must not be in the future.
    /// * `limit` - Number of documents per page
    /// * `offset` - Beginning point in the list to retrieve from.
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
    ///         .sms_campaigns
    ///         .get_sms_campaigns(
    ///             &GetSmsCampaignsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_sms_campaigns(
        &self,
        request: &GetSmsCampaignsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSmsCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "smsCampaigns",
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
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

    /// Create a new SMS campaign with the required name, sender, and content fields. The sender name is limited to 11 alphanumeric characters or 15 numeric characters, and the content should stay within 160 characters per SMS segment. If a scheduledAt date is provided, listIds in recipients become mandatory; accounts under validation are limited to 4 total campaigns and campaigns with more than 10 recipients will be saved as draft.
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
    ///         .sms_campaigns
    ///         .create_sms_campaign(
    ///             &CreateSmsCampaignRequest {
    ///                 content: "Get a discount by visiting our NY store and saying : Happy Spring!"
    ///                     .to_string(),
    ///                 name: "Spring Promo Code".to_string(),
    ///                 sender: "MyShop".to_string(),
    ///                 organisation_prefix: None,
    ///                 recipients: None,
    ///                 scheduled_at: None,
    ///                 unicode_enabled: None,
    ///                 unsubscribe_instruction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_sms_campaign(
        &self,
        request: &CreateSmsCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateSmsCampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "smsCampaigns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve detailed information about a specific SMS campaign by its ID, including campaign content, sender, recipients with list names, statistics (delivered, sent, bounces, unsubscriptions, answered), and tags. Unlike the list endpoint, recipients are returned as objects with id and name fields rather than plain IDs.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - id of the SMS campaign
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
    ///     client.sms_campaigns.get_sms_campaign(1000000, None).await;
    /// }
    /// ```
    pub async fn get_sms_campaign(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetSmsCampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("smsCampaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing SMS campaign''s properties such as name, sender, content, recipients, scheduled date, organisation prefix, and unsubscribe instructions. The request body must contain at least one valid field to update. The campaign must exist and must be of type SMS; if a scheduledAt is provided, valid recipients must be present either in the request or already configured on the campaign.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - id of the SMS campaign
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
    ///         .sms_campaigns
    ///         .update_sms_campaign(
    ///             1000000,
    ///             &UpdateSmsCampaignRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_sms_campaign(
        &self,
        campaign_id: i64,
        request: &UpdateSmsCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("smsCampaigns/{}", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an SMS campaign by its campaign ID. Only campaigns that have not been scheduled or sent can be deleted; attempting to delete a campaign that is queued, in process, or has been sent with recipients will return a 403 permission denied error.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - id of the SMS campaign
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
    ///         .sms_campaigns
    ///         .delete_sms_campaign(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_sms_campaign(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("smsCampaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// It returns the background process ID which on completion calls the notify URL that you have set in the input.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - id of the campaign
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
    ///         .sms_campaigns
    ///         .request_sms_recipient_export(
    ///             1000000,
    ///             &RequestSmsRecipientExportRequest {
    ///                 recipients_type: RequestSmsRecipientExportRequestRecipientsType::All,
    ///                 notify_url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn request_sms_recipient_export(
        &self,
        campaign_id: i64,
        request: &RequestSmsRecipientExportRequest,
        options: Option<RequestOptions>,
    ) -> Result<RequestSmsRecipientExportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("smsCampaigns/{}/exportRecipients", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send an existing SMS campaign immediately by scheduling it for the current time. The system verifies your account''s SMS credit balance before dispatching; if credits are insufficient or the remaining credit is less than the number of recipients, a 402 error is returned. The campaign must have valid recipients and content already configured.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - id of the campaign
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
    ///         .sms_campaigns
    ///         .send_sms_campaign_now(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn send_sms_campaign_now(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("smsCampaigns/{}/sendNow", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Send report of Sent and Archived campaign, to the specified email addresses, with respective data and a pdf attachment in detail.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - id of the campaign
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
    ///         .sms_campaigns
    ///         .send_sms_report(
    ///             1000000,
    ///             &SendReport {
    ///                 email: SendReportEmail {
    ///                     body: "Please find attached the report of our last email campaign.".to_string(),
    ///                     to: vec!["jim.suehan@example.com".to_string()],
    ///                     ..Default::default()
    ///                 },
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_sms_report(
        &self,
        campaign_id: i64,
        request: &SendReport,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("smsCampaigns/{}/sendReport", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send a test SMS to a specified phone number to preview the campaign before sending it to all recipients. The phone number must belong to one of your existing contacts in your Brevo account and must not be blacklisted. The number should include the country code (e.g. 33689965433).
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the SMS campaign
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
    ///         .sms_campaigns
    ///         .send_test_sms(
    ///             1000000,
    ///             &SendTestSmsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_test_sms(
        &self,
        campaign_id: i64,
        request: &SendTestSmsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("smsCampaigns/{}/sendTest", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update the status of an SMS campaign, such as suspending, archiving, or replicating it. Available status values include suspended, archive, darchive, sent, queued, replicate, replicateTemplate, cancel, and draft. Note that the replicateTemplate status is only available for template type campaigns.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - id of the campaign
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
    ///         .sms_campaigns
    ///         .update_sms_campaign_status(
    ///             1000000,
    ///             &UpdateCampaignStatus {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_sms_campaign_status(
        &self,
        campaign_id: i64,
        request: &UpdateCampaignStatus,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("smsCampaigns/{}/status", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
