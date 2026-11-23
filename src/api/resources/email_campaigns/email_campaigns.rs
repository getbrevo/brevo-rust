use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EmailCampaignsClient {
    pub http_client: HttpClient,
}

impl EmailCampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// <Note>The response payload for this endpoint has changed
    /// You now need to specify which type of statistics you would like to retrieve. For more information visit [this page](https://developers.brevo.com/changelog/2023/2/7).</Note>
    ///
    /// # Arguments
    ///
    /// * `type_` - Filter on the type of the campaigns
    /// * `status` - Filter on the status of the campaign
    /// * `statistics` - Filter on the type of statistics required. Example: **globalStats** value will only fetch globalStats info of the campaign in the returned response. This option only returns data for events that occurred in the last 6 months. For older campaigns, it is advisable to use the **Get Campaign Report** endpoint.
    /// * `start_date` - **Mandatory if endDate is used.** Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. The date range between `startDate` and `endDate` must not exceed 2 years. `startDate` must not be in the future.
    /// * `end_date` - **Mandatory if startDate is used.** Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. The date range between `startDate` and `endDate` must not exceed 2 years. `endDate` must not be in the future.
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    /// * `exclude_html_content` - Use this flag to exclude htmlContent from the response body. If set to **true**, the htmlContent field will be returned as an empty string in the response body.
    /// * `exclude_pdf_attachment` - Use this flag to filter out campaigns that have a PDF attachment. If set to **true**, only campaigns without a PDF attachment (or with no attachment at all) will be returned.
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
    ///         .email_campaigns
    ///         .get_email_campaigns(
    ///             &GetEmailCampaignsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_email_campaigns(
        &self,
        request: &GetEmailCampaignsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetEmailCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "emailCampaigns",
                None,
                QueryBuilder::new()
                    .serialize("type", request.r#type.clone())
                    .serialize("status", request.status.clone())
                    .serialize("statistics", request.statistics.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .bool("excludeHtmlContent", request.exclude_html_content.clone())
                    .bool(
                        "excludePdfAttachment",
                        request.exclude_pdf_attachment.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Create a new email campaign. The campaign requires at minimum a name and sender details, and is created in draft status by default. You must provide email content via one of three mutually exclusive options: htmlContent (inline HTML), htmlUrl (remote URL), or templateId (existing template); additionally, A/B testing can be enabled by setting abTesting to true with subjectA and subjectB, but this is incompatible with sendAtBestTime.
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
    ///         .email_campaigns
    ///         .create_email_campaign(
    ///             &CreateEmailCampaignRequest {
    ///                 name: "Newsletter - May 2017".to_string(),
    ///                 sender: CreateEmailCampaignRequestSender {
    ///                     ..Default::default()
    ///                 },
    ///                 ab_testing: None,
    ///                 attachment_url: None,
    ///                 email_expiration_date: None,
    ///                 footer: None,
    ///                 header: None,
    ///                 html_content: None,
    ///                 html_url: None,
    ///                 increase_rate: None,
    ///                 initial_quota: None,
    ///                 inline_image_activation: None,
    ///                 ip_warmup_enable: None,
    ///                 mirror_active: None,
    ///                 params: None,
    ///                 preview_text: None,
    ///                 recipients: None,
    ///                 reply_to: None,
    ///                 scheduled_at: None,
    ///                 send_at_best_time: None,
    ///                 split_rule: None,
    ///                 subject: None,
    ///                 subject_a: None,
    ///                 subject_b: None,
    ///                 tag: None,
    ///                 template_id: None,
    ///                 to_field: None,
    ///                 unsubscription_page_id: None,
    ///                 update_form_id: None,
    ///                 utm_campaign: None,
    ///                 utm_content: None,
    ///                 utm_id: None,
    ///                 utm_medium: None,
    ///                 utm_source: None,
    ///                 utm_term: None,
    ///                 winner_criteria: None,
    ///                 winner_delay: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_email_campaign(
        &self,
        request: &CreateEmailCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateEmailCampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "emailCampaigns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Upload an image to your account''s image gallery by providing an absolute URL to the image. The maximum allowed image size is 2MB and supported formats are jpeg, jpg, png, bmp, and gif; local file uploads are not supported.
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
    ///         .email_campaigns
    ///         .upload_image_to_gallery(
    ///             &UploadImageToGalleryRequest {
    ///                 image_url: "https://somedomain.com/image1.jpg".to_string(),
    ///                 name: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload_image_to_gallery(
        &self,
        request: &UploadImageToGalleryRequest,
        options: Option<RequestOptions>,
    ) -> Result<UploadImageToGalleryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "emailCampaigns/images",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve detailed information about a specific email campaign by its ID, including recipients, statistics, and HTML content. Use the statistics query parameter to select which statistics to include (globalStats, linksStats, statsByDomain, statsByDevice, or statsByBrowser); statsByDevice and statsByBrowser are only available on this single-campaign endpoint. You can exclude HTML content from the response by setting excludeHtmlContent to true.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign
    /// * `statistics` - Filter on the type of statistics required. Example: **globalStats** value will only fetch globalStats info of the campaign in the returned response. `statsByDevice` and `statsByBrowser` are only available when retrieving a single campaign (not in the list endpoint).
    /// * `exclude_html_content` - Use this flag to exclude htmlContent from the response body. If set to **true**, htmlContent field will be returned as empty string in the response body
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
    ///         .email_campaigns
    ///         .get_email_campaign(
    ///             1000000,
    ///             &GetEmailCampaignQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_email_campaign(
        &self,
        campaign_id: i64,
        request: &GetEmailCampaignQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetEmailCampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("emailCampaigns/{}", campaign_id),
                None,
                QueryBuilder::new()
                    .serialize("statistics", request.statistics.clone())
                    .bool("excludeHtmlContent", request.exclude_html_content.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update an existing email campaign''s properties such as name, subject, content, sender, recipients, schedule, and A/B testing configuration. The campaign must exist and the request body must contain at least one valid field to update. Only draft or scheduled campaigns can be modified; if sendAtBestTime is enabled, IP warmup will be automatically disabled.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign
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
    ///         .email_campaigns
    ///         .update_email_campaign(
    ///             1000000,
    ///             &UpdateEmailCampaignRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_email_campaign(
        &self,
        campaign_id: i64,
        request: &UpdateEmailCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("emailCampaigns/{}", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an email campaign by its campaign ID. Only campaigns that have not been scheduled can be deleted; attempting to delete a campaign that has already been scheduled will return a 403 permission denied error. Related data in templates, newsletter builder, and schedule collections is also cleaned up.
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
    ///         .email_campaigns
    ///         .delete_email_campaign(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_email_campaign(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("emailCampaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Obtain winning version of an A/B test email campaign
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the A/B test campaign
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
    ///         .email_campaigns
    ///         .get_ab_test_campaign_result(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_ab_test_campaign_result(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetAbTestCampaignResultResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("emailCampaigns/{}/abTestCampaignResult", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Export the recipients of a sent email campaign as an asynchronous process, filtered by recipient type (e.g. openers, clickers, hardBounces). The recipientsType field is required and determines which subset of recipients to export. An optional notifyURL webhook will be called once the export is complete, and the response returns a processId to track the export status.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign
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
    ///         .email_campaigns
    ///         .email_export_recipients(
    ///             1000000,
    ///             &EmailExportRecipientsRequest {
    ///                 recipients_type: EmailExportRecipientsRequestRecipientsType::All,
    ///                 notify_url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn email_export_recipients(
        &self,
        campaign_id: i64,
        request: &EmailExportRecipientsRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmailExportRecipientsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("emailCampaigns/{}/exportRecipients", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send an existing email campaign immediately by scheduling it for the current time. The campaign must have valid recipients and content configured before sending. The system verifies your account''s send limit and credit balance before dispatching; if credits are insufficient, a 402 error is returned.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign
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
    ///         .email_campaigns
    ///         .send_email_campaign_now(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn send_email_campaign_now(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("emailCampaigns/{}/sendNow", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// A PDF will be sent to the specified email addresses
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign
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
    ///         .email_campaigns
    ///         .send_report(
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
    pub async fn send_report(
        &self,
        campaign_id: i64,
        request: &SendReport,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("emailCampaigns/{}/sendReport", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send a test version of an email campaign to specified email addresses or your entire test list. If the emailTo array is left empty, the test mail will be sent to all addresses in your test list. You can send a maximum of 50 test emails per day.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign
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
    ///         .email_campaigns
    ///         .send_test_email(
    ///             1000000,
    ///             &SendTestEmail {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_test_email(
        &self,
        campaign_id: i64,
        request: &SendTestEmail,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("emailCampaigns/{}/sendTest", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a unique URL to share & import an email template from one Brevo account to another.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign or template
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
    ///         .email_campaigns
    ///         .get_shared_template_url(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_shared_template_url(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetSharedTemplateUrlResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("emailCampaigns/{}/sharedUrl", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the status of an email campaign, such as suspending, archiving, or replicating it. Available status values include suspended, archive, darchive, sent, queued, replicate, replicateTemplate, cancel, and draft. Note that the replicateTemplate status is only available for template type campaigns.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Id of the campaign
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
    ///         .email_campaigns
    ///         .update_campaign_status(
    ///             1000000,
    ///             &UpdateCampaignStatus {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_campaign_status(
        &self,
        campaign_id: i64,
        request: &UpdateCampaignStatus,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("emailCampaigns/{}/status", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
