use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EcommerceClient {
    pub http_client: HttpClient,
}

impl EcommerceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of all ecommerce categories stored in your Brevo account. Results are sorted by creation date in descending order by default, and can be filtered by category IDs, name, modification date, creation date, or deletion status. The response includes a `count` field with the total number of matching categories, and pagination defaults to 50 categories per page (maximum 100).
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    /// * `ids` - Filter by category ids
    /// * `name` - Filter by category name
    /// * `modified_since` - Filter (urlencoded) the categories modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `created_since` - Filter (urlencoded) the categories created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `is_deleted` - Filter categories by their deletion status. If `false` is passed, only categories that are not deleted will be returned.
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
    ///         .ecommerce
    ///         .get_categories(
    ///             &GetCategoriesQueryRequest {
    ///                 limit: None,
    ///                 offset: None,
    ///                 sort: None,
    ///                 ids: vec![],
    ///                 name: None,
    ///                 modified_since: None,
    ///                 created_since: None,
    ///                 is_deleted: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_categories(
        &self,
        request: &GetCategoriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCategoriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "categories",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .string_array("ids", request.ids.clone())
                    .string("name", request.name.clone())
                    .string("modifiedSince", request.modified_since.clone())
                    .string("createdSince", request.created_since.clone())
                    .string("isDeleted", request.is_deleted.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new ecommerce category or update an existing one, identified by the mandatory `id` field. When `updateEnabled` is set to `false` (the default), the endpoint performs an insert and returns `201`; if the category ID already exists, a `400` error is returned. When `updateEnabled` is `true`, the endpoint performs an upsert, returning `201` for a new category or `204` when an existing category is updated. The `name` field is mandatory for creation but optional for updates.
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
    ///         .ecommerce
    ///         .create_update_category(
    ///             &CreateUpdateCategoryRequest {
    ///                 id: "CAT123".to_string(),
    ///                 deleted_at: None,
    ///                 is_deleted: None,
    ///                 name: None,
    ///                 update_enabled: None,
    ///                 url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_update_category(
        &self,
        request: &CreateUpdateCategoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<CreateUpdateCategoryResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "categories",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create or update multiple ecommerce categories in a single request. The `categories` array accepts up to 100 category objects, each requiring a unique `id`. When `updateEnabled` is `false` (the default), all categories are inserted as new; if any ID already exists, a `400` error is returned. When `updateEnabled` is `true`, existing categories are updated and new ones are created via upsert. Duplicate IDs within the same request payload are rejected. The response returns the count of created and updated categories.
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
    ///         .ecommerce
    ///         .create_update_batch_category(
    ///             &CreateUpdateBatchCategoryRequest {
    ///                 categories: vec![CreateUpdateBatchCategoryRequestCategoriesItem {
    ///                     id: "CAT123".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 update_enabled: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_update_batch_category(
        &self,
        request: &CreateUpdateBatchCategoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateUpdateBatchCategoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "categories/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the full details of a single ecommerce category by its unique ID. The response includes the category name, URL, creation and modification timestamps, and deletion status. Returns a `404` error if no category matches the provided ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Category ID
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
    ///         .ecommerce
    ///         .get_category_info(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_category_info(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetCategoryDetails, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("categories/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Getting access to Brevo eCommerce.
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
    ///     client.ecommerce.activate_the_e_commerce_app(None).await;
    /// }
    /// ```
    pub async fn activate_the_e_commerce_app(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(Method::POST, "ecommerce/activate", None, None, options)
            .await
    }

    /// Retrieve aggregated ecommerce attribution metrics for one or more Brevo email campaigns, SMS campaigns, or automation workflows. You can optionally filter by a date range using `periodFrom` and `periodTo` in RFC3339 format. The response includes per-source metrics (orders count, revenue, and average basket) as well as aggregated totals across all requested sources.
    ///
    /// # Arguments
    ///
    /// * `period_from` - When getting metrics for a specific period, define the starting datetime in RFC3339 format
    /// * `period_to` - When getting metrics for a specific period, define the end datetime in RFC3339 format
    /// * `email_campaign_id_array` - The email campaign ID(s) to get metrics for
    /// * `sms_campaign_id_array` - The SMS campaign ID(s) to get metrics for
    /// * `automation_workflow_email_id_array` - The automation workflow ID(s) to get email attribution metrics for
    /// * `automation_workflow_sms_id_array` - The automation workflow ID(s) to get SMS attribution metrics for
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
    ///         .ecommerce
    ///         .get_attribution_metrics_for_one_or_more_brevo_campaigns_or_workflows(
    ///             &GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest {
    ///                 period_from: Some(DateTime::parse_from_rfc3339("2022-01-02T00:00:00Z").unwrap()),
    ///                 period_to: Some(DateTime::parse_from_rfc3339("2022-01-03T00:00:00Z").unwrap()),
    ///                 email_campaign_id_array: vec![Some("sale".to_string())],
    ///                 sms_campaign_id_array: vec![],
    ///                 automation_workflow_email_id_array: vec![],
    ///                 automation_workflow_sms_id_array: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_attribution_metrics_for_one_or_more_brevo_campaigns_or_workflows(
        &self,
        request: &GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetEcommerceAttributionMetricsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "ecommerce/attribution/metrics",
                None,
                QueryBuilder::new()
                    .datetime("periodFrom", request.period_from.clone())
                    .datetime("periodTo", request.period_to.clone())
                    .string_array("emailCampaignId[]", request.email_campaign_id_array.clone())
                    .string_array("smsCampaignId[]", request.sms_campaign_id_array.clone())
                    .string_array(
                        "automationWorkflowEmailId[]",
                        request.automation_workflow_email_id_array.clone(),
                    )
                    .string_array(
                        "automationWorkflowSmsId[]",
                        request.automation_workflow_sms_id_array.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve detailed attribution metrics for a single Brevo campaign or automation workflow, identified by its conversion source type and ID. The response includes orders count, revenue, average basket value, and the number of new customers attributed to that specific campaign or workflow.
    ///
    /// # Arguments
    ///
    /// * `conversion_source` - The Brevo campaign type or workflow type for which data will be retrieved
    /// * `conversion_source_id` - The Brevo campaign or automation workflow id for which data will be retrieved
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
    ///     client.ecommerce.get_detailed_attribution_metrics_for_a_single_brevo_campaign_or_workflow(&GetEcommerceAttributionMetricsConversionSourceConversionSourceIdRequestConversionSource::EmailCampaign, &"sale".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_detailed_attribution_metrics_for_a_single_brevo_campaign_or_workflow(
        &self,
        conversion_source: &GetEcommerceAttributionMetricsConversionSourceConversionSourceIdRequestConversionSource,
        conversion_source_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse, ApiError>
    {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "ecommerce/attribution/metrics/{}/{}",
                    conversion_source, conversion_source_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve the list of products whose sales have been attributed to a specific Brevo campaign or automation workflow. Each product entry includes its ID, name, SKU, image URL, product URL, price, revenue, and orders count. The conversion source type must be one of `email_campaign`, `sms_campaign`, `automation_workflow_email`, or `automation_workflow_sms`.
    ///
    /// # Arguments
    ///
    /// * `conversion_source` - The Brevo campaign or automation workflow type for which data will be retrieved
    /// * `conversion_source_id` - The Brevo campaign or automation workflow id for which data will be retrieved
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
    ///     client.ecommerce.get_attributed_product_sales_for_a_single_brevo_campaign_or_workflow(&GetEcommerceAttributionProductsConversionSourceConversionSourceIdRequestConversionSource::EmailCampaign, &"sale".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_attributed_product_sales_for_a_single_brevo_campaign_or_workflow(
        &self,
        conversion_source: &GetEcommerceAttributionProductsConversionSourceConversionSourceIdRequestConversionSource,
        conversion_source_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse, ApiError>
    {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "ecommerce/attribution/products/{}/{}",
                    conversion_source, conversion_source_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve the ISO 4217 display currency code currently configured for your Brevo ecommerce account. This currency is used to display monetary values across the ecommerce dashboard and reports. Returns a `403` error if ecommerce is not activated on the account.
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
    ///         .ecommerce
    ///         .get_the_iso4217compliant_display_currency_code_for_your_brevo_account(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_the_iso4217compliant_display_currency_code_for_your_brevo_account(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetEcommerceConfigDisplayCurrencyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "ecommerce/config/displayCurrency",
                None,
                None,
                options,
            )
            .await
    }

    /// Set or update the ISO 4217 display currency code for your Brevo ecommerce account. This currency determines how monetary values are displayed in the ecommerce dashboard and reports. The provided currency code must be a valid ISO 4217 code; invalid codes result in a `422` error. Returns a `403` error if ecommerce is not activated on the account.
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
    ///         .ecommerce
    ///         .set_config_display_currency(
    ///             &SetConfigDisplayCurrencyRequest {
    ///                 code: "EUR".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_config_display_currency(
        &self,
        request: &SetConfigDisplayCurrencyRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetConfigDisplayCurrencyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "ecommerce/config/displayCurrency",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the orders stored in your account. Each update sent for an order (for example a status change) is returned as a separate entry, so several entries can share the same `id`.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    /// * `secondary_sort_by_modified_at` - When `true`, orders with the same `createdAt` are then ordered by their `updatedAt`, in the same direction as `sort`. Use it to get a stable order when paging through the results with `limit` and `offset`. Default is `false`.
    /// * `modified_since` - Filter (urlencoded) the orders modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `created_since` - Filter (urlencoded) the orders created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
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
    ///         .ecommerce
    ///         .get_orders(
    ///             &GetOrdersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_orders(
        &self,
        request: &GetOrdersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "orders",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .bool(
                        "secondarySortByModifiedAt",
                        request.secondary_sort_by_modified_at.clone(),
                    )
                    .string("modifiedSince", request.modified_since.clone())
                    .string("createdSince", request.created_since.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an order or records a change to an existing order, identified by `id`. Each call is stored as a new entry for the order rather than replacing the previous one, so [Get order details](/reference/get-orders) can return several entries with the same `id` (one per status change or other update). Orders sent to this endpoint are processed as live data and can trigger automation workflows. To import past orders without triggering workflows, use [Create orders in batch](/reference/create-batch-order) with `historical: true`.
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
    ///         .ecommerce
    ///         .create_order(
    ///             &Order {
    ///                 amount: 308.42,
    ///                 created_at: "2021-07-29T20:59:23.383Z".to_string(),
    ///                 id: "14".to_string(),
    ///                 products: vec![OrderProductsItem {
    ///                     price: 99.99,
    ///                     product_id: "P1".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 status: "completed".to_string(),
    ///                 updated_at: "2021-07-30T10:59:23.383Z".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_order(
        &self,
        request: &Order,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "orders/status",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates multiple orders in one request instead of one order at a time. Each order is stored as a new entry rather than replacing a previous order with the same `id`, so [Get order details](/reference/get-orders) can return several entries with the same `id`.
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
    ///         .ecommerce
    ///         .create_batch_order(
    ///             &CreateBatchOrderRequest {
    ///                 orders: vec![Order {
    ///                     amount: 308.42,
    ///                     created_at: "2021-07-29T20:59:23.383Z".to_string(),
    ///                     id: "14".to_string(),
    ///                     products: vec![OrderProductsItem {
    ///                         price: 99.99,
    ///                         product_id: "P1".to_string(),
    ///                         ..Default::default()
    ///                     }],
    ///                     status: "completed".to_string(),
    ///                     updated_at: "2021-07-30T10:59:23.383Z".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 historical: None,
    ///                 notify_url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_batch_order(
        &self,
        request: &CreateBatchOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateBatchOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "orders/status/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a paginated list of all ecommerce products stored in your Brevo account. Results are sorted by creation date in descending order by default, and can be filtered by product IDs, name (minimum 3 characters), price range, category IDs, modification date, creation date, or deletion status. Use the `search` parameter to query across SKU, name, and ID simultaneously — results are prioritized as exact SKU match > SKU prefix match > name match > ID match. Pagination defaults to 50 products per page (maximum 1000), and the response includes a `count` field with the total number of matching products.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document in the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    /// * `ids` - Filter by product ids
    /// * `search` - Search products simultaneously across SKU, name, and ID fields. Results are returned in the following priority order: **exact SKU match** > **SKU prefix match** > **name match** > **ID match**. For example, `?search=123` on products with `{sku: "123"}` and `{sku: "123456"}` returns the exact SKU match first.
    /// * `name` - Filter by product name, minimum 3 characters should be present for search.
    /// * `price_lte` - Price filter for products less than and equals to particular amount
    /// * `price_gte` - Price filter for products greater than and equals to particular amount
    /// * `price_lt` - Price filter for products less than particular amount
    /// * `price_gt` - Price filter for products greater than particular amount
    /// * `price_eq` - Price filter for products equals to particular amount
    /// * `price_ne` - Price filter for products not equals to particular amount
    /// * `alternative_price_lte` - Alternative price filter for products less than and equals to particular amount
    /// * `alternative_price_gte` - Alternative price filter for products greater than and equals to particular amount
    /// * `alternative_price_lt` - Alternative price filter for products less than particular amount
    /// * `alternative_price_gt` - Alternative price filter for products greater than particular amount
    /// * `alternative_price_eq` - Alternative price filter for products equals to particular amount
    /// * `alternative_price_ne` - Alternative price filter for products not equals to particular amount
    /// * `categories` - Filter by categories ids
    /// * `modified_since` - Filter (urlencoded) the products modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `created_since` - Filter (urlencoded) the products created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `sort_by_field` - Sort the results by a specific field. Default sort field is `created_at` when not passed.
    /// * `is_deleted` - Filter products by their deletion status. If `false` is passed, only products that are not deleted will be returned.
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
    ///         .ecommerce
    ///         .get_products(
    ///             &GetProductsQueryRequest {
    ///                 limit: None,
    ///                 offset: None,
    ///                 sort: None,
    ///                 ids: vec![],
    ///                 search: None,
    ///                 name: None,
    ///                 price_lte: None,
    ///                 price_gte: None,
    ///                 price_lt: None,
    ///                 price_gt: None,
    ///                 price_eq: None,
    ///                 price_ne: None,
    ///                 alternative_price_lte: None,
    ///                 alternative_price_gte: None,
    ///                 alternative_price_lt: None,
    ///                 alternative_price_gt: None,
    ///                 alternative_price_eq: None,
    ///                 alternative_price_ne: None,
    ///                 categories: vec![],
    ///                 modified_since: None,
    ///                 created_since: None,
    ///                 sort_by_field: None,
    ///                 is_deleted: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_products(
        &self,
        request: &GetProductsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetProductsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "products",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .string_array("ids", request.ids.clone())
                    .string("search", request.search.clone())
                    .string("name", request.name.clone())
                    .float("price[lte]", request.price_lte.clone())
                    .float("price[gte]", request.price_gte.clone())
                    .float("price[lt]", request.price_lt.clone())
                    .float("price[gt]", request.price_gt.clone())
                    .float("price[eq]", request.price_eq.clone())
                    .float("price[ne]", request.price_ne.clone())
                    .float(
                        "alternativePrice[lte]",
                        request.alternative_price_lte.clone(),
                    )
                    .float(
                        "alternativePrice[gte]",
                        request.alternative_price_gte.clone(),
                    )
                    .float("alternativePrice[lt]", request.alternative_price_lt.clone())
                    .float("alternativePrice[gt]", request.alternative_price_gt.clone())
                    .float("alternativePrice[eq]", request.alternative_price_eq.clone())
                    .float("alternativePrice[ne]", request.alternative_price_ne.clone())
                    .string_array("categories", request.categories.clone())
                    .string("modifiedSince", request.modified_since.clone())
                    .string("createdSince", request.created_since.clone())
                    .serialize("sortByField", request.sort_by_field.clone())
                    .string("isDeleted", request.is_deleted.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new ecommerce product or update an existing one, identified by the mandatory `id` field. When `updateEnabled` is `false` (the default), the endpoint inserts a new product and returns `201`; if the product ID already exists, a `400` error is returned. When `updateEnabled` is `true`, the endpoint performs an upsert, returning `201` for a new product or `204` for an update. The `name` field is mandatory for creation but optional for updates. Product images are downloaded, validated (max 5 MB, formats: jpeg, jpg, png, bmp, gif, webp), and re-hosted on S3. The `metaInfo` object supports up to 20 keys with a cumulative size limit of approximately 1000 KB.
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
    ///         .ecommerce
    ///         .create_update_product(
    ///             &CreateUpdateProductRequest {
    ///                 id: "P11".to_string(),
    ///                 name: "Iphone 11".to_string(),
    ///                 brand: None,
    ///                 categories: None,
    ///                 deleted_at: None,
    ///                 description: None,
    ///                 image_url: None,
    ///                 is_deleted: None,
    ///                 meta_info: None,
    ///                 parent_id: None,
    ///                 price: None,
    ///                 alternative_price: None,
    ///                 sku: None,
    ///                 stock: None,
    ///                 update_enabled: None,
    ///                 url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_update_product(
        &self,
        request: &CreateUpdateProductRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<CreateUpdateProductResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "products",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create or update multiple ecommerce products in a single request. The `products` array accepts up to 100 product objects for creation (or up to 1000 when `updateEnabled` is `true` and the account has an increased limit). Each product requires a unique `id` and `name` (name is mandatory for creation only). When `updateEnabled` is `false`, all products are inserted as new; if any ID already exists, a `400` error is returned. When `updateEnabled` is `true`, existing products are updated and new ones are created via upsert. Duplicate IDs within the same request payload are rejected. The response returns the count of created and updated products.
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
    ///         .ecommerce
    ///         .create_update_batch_products(
    ///             &CreateUpdateBatchProductsRequest {
    ///                 products: vec![CreateUpdateBatchProductsRequestProductsItem {
    ///                     id: "P11".to_string(),
    ///                     name: "Iphone 11".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 update_enabled: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_update_batch_products(
        &self,
        request: &CreateUpdateBatchProductsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateUpdateBatchProductsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "products/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the full details of a single ecommerce product by its unique ID. The response includes the product name, price, SKU, URL, image URLs (original and thumbnails), categories, stock level, meta information, creation and modification timestamps, and deletion status. Returns a `404` error if no product matches the provided ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Product ID
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
    ///         .ecommerce
    ///         .get_product_info(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_product_info(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetProductDetails, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("products/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Register a contact to receive an alert for a specific product event, such as `back_in_stock`. At least one contact identifier (`ext_id`, `email`, or `sms`) must be provided; when multiple are given, priority is `ext_id` > `email` > `sms`. Returns a `404` error if the product ID does not exist, and a `403` error if product alerts are not enabled for the account.
    ///
    /// # Arguments
    ///
    /// * `id` - Product ID
    /// * `type_` - Alert type
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
    ///         .ecommerce
    ///         .create_product_alert(
    ///             &"id".to_string(),
    ///             &"back_in_stock".to_string(),
    ///             &CreateProductAlertRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_product_alert(
        &self,
        id: &str,
        type_: &str,
        request: &CreateProductAlertRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("products/{}/alerts/{}", id, type_),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
