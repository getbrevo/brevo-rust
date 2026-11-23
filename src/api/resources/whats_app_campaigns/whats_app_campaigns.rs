use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WhatsAppCampaignsClient {
    pub http_client: HttpClient,
}

impl WhatsAppCampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of all your WhatsApp campaigns with their statistics and metadata. Results can be filtered by creation date range using startDate and endDate, with a default limit of 50 and maximum of 100 per page. The sort order defaults to descending by modification date.
    ///
    /// # Arguments
    ///
    /// * `start_date` - **Mandatory if endDate is used**. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the WhatsApp campaigns created. **Prefer to pass your timezone in date-time format for accurate result**
    /// * `end_date` - **Mandatory if startDate is used**. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the WhatsApp campaigns created. **Prefer to pass your timezone in date-time format for accurate result**
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page
    /// * `sort` - Sort the results in the ascending/descending order of record modification. Default order is **descending** if `sort` is not passed
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
    ///         .whats_app_campaigns
    ///         .get_whats_app_campaigns(
    ///             &GetWhatsAppCampaignsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_whats_app_campaigns(
        &self,
        request: &GetWhatsAppCampaignsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetWhatsAppCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "whatsappCampaigns",
                None,
                QueryBuilder::new()
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

    /// <Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
    /// [Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
    /// <Note>This API requires the List and Segment ids as recipients in Body params.You can use the below Contact endpoints to get the required information.
    /// [Get all the Lists](https://developers.brevo.com/reference/getlists-1)
    /// [Get all the Segments](https://developers.brevo.com/reference/getsegments)</Note>
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
    ///         .whats_app_campaigns
    ///         .create_whats_app_campaign(
    ///             &CreateWhatsAppCampaignRequest {
    ///                 name: "Test Campaign".to_string(),
    ///                 recipients: CreateWhatsAppCampaignRequestRecipients {
    ///                     ..Default::default()
    ///                 },
    ///                 scheduled_at: "2017-06-01T12:30:00+02:00".to_string(),
    ///                 template_id: 19,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_whats_app_campaign(
        &self,
        request: &CreateWhatsAppCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateWhatsAppCampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "whatsappCampaigns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
    /// [Activating WhatsApp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
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
    ///     client.whats_app_campaigns.get_whats_app_config(None).await;
    /// }
    /// ```
    pub async fn get_whats_app_config(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetWhatsAppConfigResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "whatsappCampaigns/config", None, None, options)
            .await
    }

    /// <Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
    /// [Activating WhatsApp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
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
    ///         .whats_app_campaigns
    ///         .create_whats_app_template(
    ///             &CreateWhatsAppTemplateRequest {
    ///                 body_text: "making it look like readable English".to_string(),
    ///                 category: CreateWhatsAppTemplateRequestCategory::Marketing,
    ///                 language: "en".to_string(),
    ///                 name: "Test template".to_string(),
    ///                 header_text: None,
    ///                 media_url: None,
    ///                 source: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_whats_app_template(
        &self,
        request: &CreateWhatsAppTemplateRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateWhatsAppTemplateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "whatsappCampaigns/template",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a paginated list of all your WhatsApp templates with their status, category, language, and metadata. Results can be filtered by creation date range and optionally by source (Automation or Conversations), with a default limit of 50 and maximum of 100 per page. The sort order defaults to descending by modification date.
    ///
    /// # Arguments
    ///
    /// * `start_date` - **Mandatory if endDate is used**. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the templates created. **Prefer to pass your timezone in date-time format for accurate result**
    /// * `end_date` - **Mandatory if startDate is used**. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the templates created. **Prefer to pass your timezone in date-time format for accurate result**
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page
    /// * `sort` - Sort the results in the ascending/descending order of record modification. Default order is **descending** if `sort` is not passed
    /// * `source` - source of the template
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
    ///         .whats_app_campaigns
    ///         .get_whats_app_templates(
    ///             &GetWhatsAppTemplatesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_whats_app_templates(
        &self,
        request: &GetWhatsAppTemplatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetWhatsAppTemplatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "whatsappCampaigns/template-list",
                None,
                QueryBuilder::new()
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("source", request.source.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
    /// [Activating WhatsApp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
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
    ///         .whats_app_campaigns
    ///         .send_whats_app_template_approval(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn send_whats_app_template_approval(
        &self,
        template_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("whatsappCampaigns/template/approval/{}", template_id),
                None,
                None,
                options,
            )
            .await
    }

    /// <Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
    /// [Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
    /// <Note>This API requires the List and Segment ids as recipients in Body params.You can use the below Contact endpoints to get the required information.
    /// [Get all the Lists](https://developers.brevo.com/reference/getlists-1)
    /// [Get all the Segments](https://developers.brevo.com/reference/getsegments)</Note>
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
    ///         .whats_app_campaigns
    ///         .get_whats_app_campaign(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_whats_app_campaign(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetWhatsAppCampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("whatsappCampaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// <Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
    /// [Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
    /// <Note>This API requires the List and Segment ids as recipients in Body params.You can use the below Contact endpoints to get the required information.
    /// [Get all the Lists](https://developers.brevo.com/reference/getlists-1)
    /// [Get all the Segments](https://developers.brevo.com/reference/getsegments)</Note>
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
    ///         .whats_app_campaigns
    ///         .update_whats_app_campaign(
    ///             1000000,
    ///             &UpdateWhatsAppCampaignRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_whats_app_campaign(
        &self,
        campaign_id: i64,
        request: &UpdateWhatsAppCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("whatsappCampaigns/{}", campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a WhatsApp campaign by its campaign ID. The campaign must exist; if the campaign ID is not found, a 404 error is returned. This action is permanent and cannot be undone.
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
    ///         .whats_app_campaigns
    ///         .delete_whats_app_campaign(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_whats_app_campaign(
        &self,
        campaign_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("whatsappCampaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }
}
