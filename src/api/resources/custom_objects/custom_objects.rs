use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CustomObjectsClient {
    pub http_client: HttpClient,
}

impl CustomObjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// <Note title="Enterprise access only">Custom objects are only available to Enterprise plans.
    /// This feature is in beta. These are subject to change.</Note>
    /// Performs bulk create or update (upsert) operations for object records in a single asynchronous request. This endpoint is optimized for high-volume data imports and synchronization scenarios.
    ///
    /// **How Upsert Works:**
    /// - **Create**: Omit `identifiers`, or provide only `ext_id` (if it doesn't already exist). A new record is created with a Brevo-generated `id`.
    /// - **Update**: Provide `id` (Brevo internal ID) or an `ext_id` that already exists. The matching record is updated with the new attribute values.
    /// - **Important:** `id` is for **updates only**. Providing an `id` that does not belong to an existing record will fail during async processing (the HTTP response will still be 202, but the record will be rejected in the background). To create a new record with a stable external reference, use `ext_id` instead.
    ///
    /// **Request Structure:**
    /// Each object record in the `records` array can include:
    /// - `identifiers`: Either `id` (internal Brevo ID) or `ext_id` (your external system ID) — required for updates. **Note:** use `id` (singular), not `ids`.
    /// - `attributes`: Key-value pairs where each key is the attribute **key** (e.g., `company_name`), not the attribute label (e.g., "Company Name").
    /// - `associations`: Controls linking and unlinking of associated records (optional). Each entry specifies:
    /// - `object_type`: The type of the associated object
    /// - `action`: `link` (default) to create the association, or `unlink` to remove it
    /// - `records`: The associated records to link or unlink (each identified by `ext_id` or `id`)
    /// - **Unlink is idempotent** — unlinking a non-existing association is a no-op (no error returned)
    /// - `link` and `unlink` actions can be submitted for the same `object_type` in a single record entry
    /// - Both associated records must already exist before a link can be created
    ///
    /// > **Common mistake:** Passing the attribute **label** (the display name you see in the UI) instead of the attribute **key** will cause the attribute to be silently ignored and the record may not be created as expected.
    ///
    /// **Asynchronous Processing:**
    /// - Returns immediately with a `processId` (HTTP 202 Accepted)
    /// - Use the processId to track status via the Get process API
    ///
    /// **API and Schema Limitations:**
    /// - Max 1000 object records per request
    /// - Max request body size: 1 MB
    /// - Max 500 attributes per object record (matches the schema limit of 500 attributes per object)
    /// - Unknown attribute keys are silently ignored (no error, no attribute creation)
    /// - Max 10 association records per associated object-type in each record of the request. If you need more, send multiple requests.
    ///
    /// **Important Behaviors:**
    /// - The object schema must be created before upserting records
    /// - Unknown attribute keys are silently ignored (no error, no creation)
    /// - Both associated object records must already exist before creating a link association
    /// - Unlink operations are idempotent: attempting to unlink a non-existing association returns success
    /// - `link` and `unlink` actions can be submitted for the same `object_type` in a single record entry
    /// - Contact objects cannot be created via this endpoint
    /// - For `category` and `multiple_category` attributes, pass the option **key** as the value (not the option label or option ID).
    /// - The `id` identifier (internal Brevo ID) can only be used for **updating** existing records. To create new records, either omit identifiers (Brevo auto-generates an ID) or provide an `ext_id`.
    ///
    /// **Errors:**
    /// - Make sure both object records exist before associating them, else the API will return an error.
    /// - This route does not create objects. The object where the object records are upserted by this API must be created already else the API will return an error "invalid object type".
    ///
    /// # Arguments
    ///
    /// * `object_type` - Object type for the records to upsert. Must be a previously created custom object type. Only lowercase alphanumeric characters and underscores are allowed (max 32 characters).
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
    ///     client.custom_objects.upsertrecords(&"vehicle".to_string(), &UpsertrecordsRequest {
    ///         records: vec![UpsertrecordsRequestRecordsItem {
    ///             associations: Some(vec![UpsertrecordsRequestRecordsItemAssociationsItem {
    ///                 object_type: "garage".to_string(),
    ///                 action: Some(UpsertrecordsRequestRecordsItemAssociationsItemAction::Link),
    ///                 records: vec![UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem {
    ///                     identifiers: Some(UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemIdentifiers {
    ///                         id: Some(435435),
    ///                         ..Default::default()
    ///                     }),
    ///                     ..Default::default()
    ///                 }],
    ///                 ..Default::default()
    ///             }, UpsertrecordsRequestRecordsItemAssociationsItem {
    ///                 object_type: "garage".to_string(),
    ///                 action: Some(UpsertrecordsRequestRecordsItemAssociationsItemAction::Unlink),
    ///                 records: vec![UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem {
    ///                     identifiers: Some(UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemIdentifiers {
    ///                         ext_id: Some("old-garage-001".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     ..Default::default()
    ///                 }],
    ///                 ..Default::default()
    ///             }]),
    ///             attributes: Some(HashMap::from([("make".to_string(), serde_json::json!("Toyota")), ("model".to_string(), serde_json::json!("Camry")), ("year".to_string(), serde_json::json!(2020)), ("engine_type".to_string(), serde_json::json!("hybrid"))])),
    ///             identifiers: Some(UpsertrecordsRequestRecordsItemIdentifiers {
    ///                 ext_id: Some("VIN123".to_string()),
    ///                 ..Default::default()
    ///             }),
    ///             ..Default::default()
    ///         }]
    ///     }, None).await;
    /// }
    /// ```
    pub async fn upsertrecords(
        &self,
        object_type: &str,
        request: &UpsertrecordsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpsertrecordsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("objects/{}/batch/upsert", object_type),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note title="Enterprise access only">Custom objects are only available to Enterprise plans.
    /// This feature is in beta. These are subject to change.</Note>
    /// This API retrieves a list of object records along with their associated records and provides the total count of records for the specified object. **Note**: Contact as object type is not supported in this endpoint.
    ///
    /// # Arguments
    ///
    /// * `object_type` - Object type for the records to retrieve. Must be a previously created custom object type. Contact as object type is not supported in this endpoint.
    /// * `limit` - Number of records returned per page
    /// * `page_num` - Page number for pagination. It's used to fetch the object records on a provided page number. Must be a valid positive integer.
    /// * `sort` - Sort order, must be 'asc' or 'desc'. Default to 'desc' if not provided.
    /// * `association` - Whether to include associations, must be 'true' or 'false'. Default to 'false' if not provided.
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
    ///         .custom_objects
    ///         .getrecords(
    ///             &"vehicle".to_string(),
    ///             &GetrecordsQueryRequest {
    ///                 limit: 1000000,
    ///                 page_num: 1000000,
    ///                 sort: None,
    ///                 association: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn getrecords(
        &self,
        object_type: &str,
        request: &GetrecordsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetrecordsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("objects/{}/records", object_type),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("page_num", request.page_num.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("association", request.association.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Use this endpoint to delete multiple object records of the same object-type in one request.
    /// The request is accepted and processed asynchronously. You can track the status of the deletion process using the returned **processId**.
    /// **Limitations:** - Each request can contain up to **1000** object record identifiers - Either `ids` or `ext_ids` must be provided, but **not both** in the same request - Deletion of Brevo standard object records is not supported via this endpoint - If more records must be deleted, send multiple batch requests
    ///
    /// # Arguments
    ///
    /// * `object_type` - Object type for the records to delete
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
    ///     client.custom_objects.batch_delete_object_records(&"vehicle".to_string(), &BatchDeleteObjectRecordsRequest {
    ///         identifiers: Some(BatchDeleteObjectRecordsRequestIdentifiers::BatchDeleteObjectRecordsRequestIdentifiersIds(BatchDeleteObjectRecordsRequestIdentifiersIds {
    ///             ids: Default::default(),
    ///             ..Default::default()
    ///         })),
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn batch_delete_object_records(
        &self,
        object_type: &str,
        request: &BatchDeleteObjectRecordsRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchDeleteObjectRecordsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("objects/{}/batch/delete", object_type),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note title="Enterprise access only">Custom objects are only available to Enterprise plans.
    /// This feature is in beta. These are subject to change.</Note>
    /// Returns the records associated with a single source record. Associations of every type are returned together in one paginated list, ordered by association creation time with the most recently created association first.
    ///
    /// **Identifying the source record**
    /// Provide exactly one of `id`, `ext_id`, `email` or `sms`. Passing none of them, or more than one, returns `400`. `email` and `sms` are only accepted when `object_type` is `contact`; using either with any other object type returns `400`.
    ///
    /// **Object types**
    /// Use the object type exactly as it is defined in your account, for example `vehicle` for a custom object of that name, or `contact` for contacts. An object type that does not exist in your account returns `400`.
    ///
    /// **Filtering by associated object type**
    /// Use `type` to restrict the response to one or more associated object types, for example `?type=contact&type=garage`. Up to 5 types can be requested per call; more returns `400`. When `type` is omitted, associations of every type are returned.
    ///
    /// **Pagination**
    /// Results are returned 20 per page. The page size is fixed and cannot be changed. Increase `offset` by 20 to walk through the pages until `has_more` is `false`. An `offset` beyond the last record returns an empty `items` array with `has_more` set to `false`.
    ///
    /// **Working with contacts**
    /// - `contact` is supported both as the source `object_type` and as an associated object type.
    /// - An `id`, `ext_id`, `email` or `sms` that matches no contact returns `404`.
    /// - If several contacts share the same `ext_id`, `email` or `sms`, identify the contact by `id` to be sure of which one is used.
    /// - Contacts returned in `items` carry all of the contact's attributes, with attribute keys in lowercase — `email`, `first_name`, `last_name`, `sms`, `ext_id`, and any other contact attribute lowercased.
    /// - For contacts, `ext_id`, `created_at` and `updated_at` are not returned on `object`. A contact's external ID is available as `attributes.ext_id` when it is set.
    ///
    /// # Arguments
    ///
    /// * `object_type` - Object type of the source record, exactly as defined in your account. Accepts any object type defined in the account, for example a custom object type or `contact`.
    /// * `id` - Internal Brevo ID of the source record. Must be a positive integer. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    /// * `ext_id` - External ID of the source record in your system. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    /// * `email` - Email address of the source contact. Only accepted when `object_type` is `contact`. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    /// * `sms` - Phone number of the source contact, including the country code. It may be given with or without a leading `+`; percent-encode the `+` as `%2B`, because a literal `+` in a query string is read as a space. Only accepted when `object_type` is `contact`. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    /// * `type_` - Restricts the response to the given associated object types. Repeat the parameter to request several types, for example `?type=contact&type=garage`. Maximum 5 types per call. Associations of every type are returned when omitted.
    /// * `offset` - Number of records to skip before the first record of the page. Defaults to 0. Increase by 20 to fetch the next page.
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
    ///         .custom_objects
    ///         .get_associated_records(
    ///             &"vehicle".to_string(),
    ///             &GetAssociatedRecordsQueryRequest {
    ///                 id: Some(16789),
    ///                 ext_id: Some("507f1f77bc".to_string()),
    ///                 email: Some("jane.doe@example.com".to_string()),
    ///                 sms: Some("33612345678".to_string()),
    ///                 offset: Some(0),
    ///                 r#type: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_associated_records(
        &self,
        object_type: &str,
        request: &GetAssociatedRecordsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAssociatedRecordsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("objects/{}/associated-records", object_type),
                None,
                QueryBuilder::new()
                    .int("id", request.id.clone())
                    .string("ext_id", request.ext_id.clone())
                    .string("email", request.email.clone())
                    .string("sms", request.sms.clone())
                    .string_array("type", request.r#type.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }
}
