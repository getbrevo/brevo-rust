use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DealsClient {
    pub http_client: HttpClient,
}

impl DealsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve the list of all attributes defined for deals, including both system-default and custom attributes. Each attribute includes its label, internal name, type, required status, and available options for select-type attributes.
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
    ///     client.deals.get_deal_attributes(None).await;
    /// }
    /// ```
    pub async fn get_deal_attributes(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetCrmAttributesDealsResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "crm/attributes/deals", None, None, options)
            .await
    }

    /// Retrieve a paginated list of deals with optional filtering, sorting, and search capabilities. Results can be filtered by attributes such as deal name or owner, linked companies, linked contacts, or modification/creation timestamps. Default sort order is descending by creation date.
    ///
    /// # Arguments
    ///
    /// * `filters_attributes_deal_name` - Filter by attributes. If you have a filter for the owner on your end, please send it as filters[attributes.deal_owner] and utilize the account email for the filtering.
    /// * `filters_attributes_deal_owner` - Filter by the deal owner. Pass the account email address of the deal owner.
    /// * `filters_attributes_deal_stage` - Filter by the deal stage. Pass the stage id, retrievable from GET /crm/pipeline/details/{pipelineID}.
    /// * `filters_attributes_pipeline` - Filter by the pipeline. Pass the pipeline id, retrievable from GET /crm/pipeline/details/{pipelineID}.
    /// * `filters_linked_companies_ids` - Filter by linked companies ids
    /// * `filters_linked_contacts_ids` - Filter by linked contacts ids
    /// * `modified_since` - Filter (urlencoded) the deals modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    /// * `created_since` - Filter (urlencoded) the deals created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    /// * `offset` - Index of the first document of the page
    /// * `limit` - Number of documents per page
    /// * `sort` - Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    /// * `sort_by` - The field used to sort field names.
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
    ///         .deals
    ///         .get_all_deals(
    ///             &GetAllDealsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_all_deals(
        &self,
        request: &GetAllDealsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCrmDealsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "crm/deals",
                None,
                QueryBuilder::new()
                    .string(
                        "filters[attributes.deal_name]",
                        request.filters_attributes_deal_name.clone(),
                    )
                    .string(
                        "filters[attributes.deal_owner]",
                        request.filters_attributes_deal_owner.clone(),
                    )
                    .string(
                        "filters[attributes.deal_stage]",
                        request.filters_attributes_deal_stage.clone(),
                    )
                    .string(
                        "filters[attributes.pipeline]",
                        request.filters_attributes_pipeline.clone(),
                    )
                    .string(
                        "filters[linkedCompaniesIds]",
                        request.filters_linked_companies_ids.clone(),
                    )
                    .string(
                        "filters[linkedContactsIds]",
                        request.filters_linked_contacts_ids.clone(),
                    )
                    .string("modifiedSince", request.modified_since.clone())
                    .string("createdSince", request.created_since.clone())
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .serialize("sort", request.sort.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new deal in the CRM with the specified name, attributes, and optional associations to contacts and companies. You can assign the deal to a specific pipeline and stage by providing `pipeline` and `deal_stage` attribute IDs, which can be retrieved from the pipeline details endpoint.
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
    ///         .deals
    ///         .create_a_deal(
    ///             &PostCrmDealsRequest {
    ///                 name: "Deal: Connect with company".to_string(),
    ///                 attributes: None,
    ///                 linked_companies_ids: None,
    ///                 linked_contacts_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_a_deal(
        &self,
        request: &PostCrmDealsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCrmDealsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "crm/deals",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Import deals from a CSV file with mapping options.
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
    ///         .deals
    ///         .import_deals_creation_and_updation(
    ///             &ImportDealsCreationAndUpdationRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 mapping: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn import_deals_creation_and_updation(
        &self,
        request: &ImportDealsCreationAndUpdationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCrmDealsImportResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "crm/deals/import",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Link or unlink contacts and companies with a specific deal in a single request. You can simultaneously link new contacts/companies and unlink existing ones by providing the respective ID arrays in the request body.
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
    ///         .deals
    ///         .link_and_unlink_a_deal_with_contacts_and_companies(
    ///             &"id".to_string(),
    ///             &PatchCrmDealsLinkUnlinkIdRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn link_and_unlink_a_deal_with_contacts_and_companies(
        &self,
        id: &str,
        request: &PatchCrmDealsLinkUnlinkIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("crm/deals/link-unlink/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the full details of a single deal by its identifier, including its attributes, pipeline stage, linked contacts, and linked companies. Returns a 404 error if the deal does not exist.
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
    ///     client.deals.get_a_deal(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_a_deal(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Deal, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("crm/deals/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a deal by its identifier. The requesting user must be the deal owner or have manage permission on deals; otherwise, a 403 Forbidden error is returned.
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
    ///     client.deals.delete_a_deal(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_a_deal(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("crm/deals/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing deal''s attributes, name, linked contacts, or linked companies. Note that passing `linkedContactsIds` or `linkedCompaniesIds` replaces the entire list of associations, so omitted IDs will be removed. To move a deal to a different pipeline or stage, provide both the `pipeline` and `deal_stage` attribute IDs.
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
    ///         .deals
    ///         .update_a_deal(
    ///             &"id".to_string(),
    ///             &PatchCrmDealsIdRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_a_deal(
        &self,
        id: &str,
        request: &PatchCrmDealsIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("crm/deals/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint is deprecated. Prefer /crm/pipeline/details/{pipelineID} instead.
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
    ///     client.deals.get_pipeline_stages(None).await;
    /// }
    /// ```
    pub async fn get_pipeline_stages(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Pipeline, ApiError> {
        self.http_client
            .execute_request(Method::GET, "crm/pipeline/details", None, None, options)
            .await
    }

    /// Retrieve the list of all deal pipelines configured for your account, including each pipeline''s stages and settings. If no pipelines have been configured yet, a default pipeline is automatically created and returned.
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
    ///     client.deals.get_all_pipelines(None).await;
    /// }
    /// ```
    pub async fn get_all_pipelines(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Pipelines, ApiError> {
        self.http_client
            .execute_request(Method::GET, "crm/pipeline/details/all", None, None, options)
            .await
    }

    /// Retrieve the details of a specific deal pipeline by its identifier, including its stages, stage ordering, and configuration. Use this endpoint to obtain the pipeline and stage IDs needed when creating or updating deals.
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
    ///         .deals
    ///         .get_a_pipeline(&"pipelineID".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_a_pipeline(
        &self,
        pipeline_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Pipelines, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("crm/pipeline/details/{}", pipeline_id),
                None,
                None,
                options,
            )
            .await
    }
}
