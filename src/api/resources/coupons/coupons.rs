use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CouponsClient {
    pub http_client: HttpClient,
}

impl CouponsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of all coupon collections in your Brevo account. Results can be sorted by creation date, remaining coupons count, or expiration date, in ascending or descending order. Pagination defaults to 50 collections per page (maximum 100).
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents returned per page
    /// * `offset` - Index of the first document on the page
    /// * `sort` - Sort the results by creation time in ascending/descending order
    /// * `sort_by` - The field used to sort coupon collections
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
    ///         .coupons
    ///         .get_coupon_collections(
    ///             &GetCouponCollectionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_coupon_collections(
        &self,
        request: &GetCouponCollectionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCouponCollection, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "couponCollections",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new coupon collection with a name and a default coupon value. You can optionally set an expiration date in RFC3339 format and configure alert thresholds to receive email notifications when remaining coupons or remaining days before expiration fall below a specified number. The collection ID is auto-generated as a UUID and returned in the response.
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
    ///         .coupons
    ///         .create_coupon_collection(
    ///             &CreateCouponCollectionRequest {
    ///                 default_coupon: "Winter".to_string(),
    ///                 name: "10%OFF".to_string(),
    ///                 expiration_date: None,
    ///                 remaining_coupons_alert: None,
    ///                 remaining_days_alert: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_coupon_collection(
        &self,
        request: &CreateCouponCollectionRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateCouponCollectionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "couponCollections",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the details of a single coupon collection by its UUID. The response includes the collection name, default coupon value, total and remaining coupon counts, and creation timestamp. Returns a `404` error if no collection matches the provided ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the collection to return
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
    ///         .coupons
    ///         .get_coupon_collection(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_coupon_collection(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetCouponCollection, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("couponCollections/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing coupon collection by its UUID. You can modify the default coupon value, set or remove the expiration date (pass `null` to remove), and configure or disable alert thresholds for remaining coupons or remaining days. Only the fields included in the request body are updated; omitted fields remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the collection to update
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
    ///         .coupons
    ///         .update_coupon_collection(
    ///             &"id".to_string(),
    ///             &UpdateCouponCollectionRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_coupon_collection(
        &self,
        id: &str,
        request: &UpdateCouponCollectionRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateCouponCollectionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("couponCollections/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Add coupons to an existing coupon collection. The `coupons` array must contain between 1 and 10,000 unique coupon code strings, all associated with the specified `collectionId`. Coupon creation is processed asynchronously and a `204` status is returned immediately upon acceptance. Returns a `404` error if the specified coupon collection does not exist.
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
    ///         .coupons
    ///         .create_coupons(
    ///             &CreateCouponsRequest {
    ///                 collection_id: "23befbae-1505-47a8-bd27-e30ef739f32c".to_string(),
    ///                 coupons: vec!["Uf12AF".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_coupons(
        &self,
        request: &CreateCouponsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "coupons",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
