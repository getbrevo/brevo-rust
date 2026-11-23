use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CompaniesClient {
    pub http_client: HttpClient,
}

impl CompaniesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of companies with optional filtering, sorting, and search capabilities. Results are sorted by creation date in descending order by default, and can be filtered by attributes, linked contacts, linked deals, or modification/creation timestamps.
    ///
    /// # Arguments
    ///
    /// * `filters_attributes_name` - Filter by attributes. If you have a filter for the owner on your side please send it as filters[attributes.owner] and utilize the account email for the filtering.
    /// * `linked_contacts_ids` - Filter by linked contacts ids
    /// * `linked_deals_ids` - Filter by linked Deals ids
    /// * `modified_since` - Filter (urlencoded) the companies modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    /// * `created_since` - Filter (urlencoded) the companies created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    /// * `page` - Index of the first document of the page
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
    ///         .companies
    ///         .get_all_companies(
    ///             &GetAllCompaniesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_all_companies(
        &self,
        request: &GetAllCompaniesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCompaniesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "companies",
                None,
                QueryBuilder::new()
                    .string(
                        "filters[attributes.name]",
                        request.filters_attributes_name.clone(),
                    )
                    .int("linkedContactsIds", request.linked_contacts_ids.clone())
                    .string("linkedDealsIds", request.linked_deals_ids.clone())
                    .string("modifiedSince", request.modified_since.clone())
                    .string("createdSince", request.created_since.clone())
                    .int("page", request.page.clone())
                    .int("limit", request.limit.clone())
                    .serialize("sort", request.sort.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new CRM company with the specified name, attributes, and optional associations to contacts and deals. The company name is required, and you can optionally provide a country code when a phone number attribute is included.
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
    ///         .companies
    ///         .create_a_company(
    ///             &PostCompaniesRequest {
    ///                 name: "company".to_string(),
    ///                 attributes: None,
    ///                 country_code: None,
    ///                 linked_contacts_ids: None,
    ///                 linked_deals_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_a_company(
        &self,
        request: &PostCompaniesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCompaniesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "companies",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Import companies from a CSV file with mapping options.
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
    ///         .companies
    ///         .import_companies_creation_and_updation(
    ///             &ImportCompaniesCreationAndUpdationRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 mapping: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn import_companies_creation_and_updation(
        &self,
        request: &ImportCompaniesCreationAndUpdationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCompaniesImportResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "companies/import",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Link or unlink contacts and deals with a specific company in a single request. You can simultaneously link new contacts/deals and unlink existing ones by providing the respective ID arrays in the request body.
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
    ///         .companies
    ///         .link_and_unlink_company_with_contact_and_deal(
    ///             &"id".to_string(),
    ///             &PatchCompaniesLinkUnlinkIdRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn link_and_unlink_company_with_contact_and_deal(
        &self,
        id: &str,
        request: &PatchCompaniesLinkUnlinkIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("companies/link-unlink/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the full details of a single company by its identifier, including its attributes, linked contacts, and linked deals. Returns a 404 error if the company does not exist, or a 403 error if the user lacks permission to view the company.
    ///
    /// # Arguments
    ///
    /// * `id` - Get Company Details
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
    ///         .companies
    ///         .get_a_company(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_a_company(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Company, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("companies/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a company by its identifier. The requesting user must be the company owner or have manage permission on companies; otherwise, a 403 Forbidden error is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - Company ID to delete
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
    ///         .companies
    ///         .delete_a_company(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_a_company(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("companies/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing company''s attributes, name, linked contacts, or linked deals. Note that passing `linkedContactsIds` or `linkedDealsIds` replaces the entire list of associations, so omitted IDs will be removed. The company name cannot be set to an empty string.
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
    ///         .companies
    ///         .update_a_company(
    ///             &"id".to_string(),
    ///             &PatchCompaniesIdRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_a_company(
        &self,
        id: &str,
        request: &PatchCompaniesIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<Company, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("companies/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a new custom attribute for companies or deals. The attribute label must be unique within the object type, cannot exceed 50 characters, and cannot use reserved names. For `single-select` or `multi-choice` attribute types, you must also provide the `optionsLabels` array.
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
    ///         .companies
    ///         .create_a_company_deal_attribute(
    ///             &PostCrmAttributesRequest {
    ///                 attribute_type: PostCrmAttributesRequestAttributeType::Text,
    ///                 label: "Attribute Label".to_string(),
    ///                 object_type: PostCrmAttributesRequestObjectType::Companies,
    ///                 description: None,
    ///                 options_labels: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_a_company_deal_attribute(
        &self,
        request: &PostCrmAttributesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCrmAttributesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "crm/attributes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an existing custom attribute by its identifier. This permanently removes the attribute definition and cleans up all references to it across companies or deals. System-default and non-editable attributes cannot be deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - Attribute ID
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
    ///         .companies
    ///         .delete_an_attribute(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_an_attribute(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("crm/attributes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing custom attribute''s label or options. You can rename the attribute label or modify the available options for `single-select` and `multi-choice` attribute types. System-default attributes cannot be modified except for specific editable fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Attribute ID
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
    ///         .companies
    ///         .update_an_attribute(
    ///             &"id".to_string(),
    ///             &PatchCrmAttributesIdRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_an_attribute(
        &self,
        id: &str,
        request: &PatchCrmAttributesIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("crm/attributes/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the list of all attributes defined for companies, including both system-default and custom attributes. Each attribute includes its label, internal name, type, required status, and available options for select-type attributes.
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
    ///     client.companies.get_company_attributes(None).await;
    /// }
    /// ```
    pub async fn get_company_attributes(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetCrmAttributesCompaniesResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "crm/attributes/companies", None, None, options)
            .await
    }
}
