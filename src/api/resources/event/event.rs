use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EventClient {
    pub http_client: HttpClient,
}

impl EventClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// <Note>
    /// This endpoint currently only supports custom events.
    /// </Note>
    ///
    /// Retrieve a list of events filtered by various criteria.
    ///
    /// # Arguments
    ///
    /// * `contact_id` - Filter by contact ID (repeatable)
    /// * `event_name` - Filter by event name (repeatable)
    /// * `object_type` - Filter by object type (repeatable)
    /// * `start_date` - Mandatory if endDate is used. Start of date range (YYYY-MM-DD or RFC3339). Defaults to 6 months ago when omitted alongside endDate. Must be ≤ endDate.
    /// * `end_date` - Mandatory if startDate is used. End of date range (YYYY-MM-DD or RFC3339). Must be ≥ startDate.
    /// * `limit` - Max events to return. Default 100, min 1, max 10000.
    /// * `offset` - Events to skip for pagination. Default 0, min 0.
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
    ///         .event
    ///         .get_events(
    ///             &GetEventsQueryRequest {
    ///                 contact_id: vec![],
    ///                 event_name: vec![],
    ///                 object_type: vec![],
    ///                 start_date: None,
    ///                 end_date: None,
    ///                 limit: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_events(
        &self,
        request: &GetEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetEventsList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "events",
                None,
                QueryBuilder::new()
                    .int_array("contact_id", request.contact_id.clone())
                    .string_array("event_name", request.event_name.clone())
                    .string_array("object_type", request.object_type.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create an event to track a contact's interaction. Events created with this endpoint are stored as custom events, including events named `cart_updated`, `cart_deleted` or `order_completed`, so they don't trigger ecommerce automations such as abandoned cart. To send ecommerce cart events, activate the ecommerce app and use the Brevo tracker or the [trackEvent endpoint](/docs/track-custom-events-rest).
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
    ///         .event
    ///         .create_event(
    ///             &CreateEventRequest {
    ///                 event_name: "video_played".to_string(),
    ///                 identifiers: CreateEventRequestIdentifiers {
    ///                     ..Default::default()
    ///                 },
    ///                 contact_properties: None,
    ///                 event_date: None,
    ///                 event_properties: None,
    ///                 object: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_event(
        &self,
        request: &CreateEventRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "events",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create multiple events to track contacts' interactions in a single request. Events created with this endpoint are stored as custom events, including events named `cart_updated`, `cart_deleted` or `order_completed`, so they don't trigger ecommerce automations such as abandoned cart. To send ecommerce cart events, activate the ecommerce app and use the Brevo tracker or the [trackEvent endpoint](/docs/track-custom-events-rest).
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
    ///         .event
    ///         .create_batch_events(
    ///             &CreateBatchEventsRequest {
    ///                 events: vec![CreateBatchEventsRequestEventsItem {
    ///                     event_name: "order_created".to_string(),
    ///                     identifiers: CreateBatchEventsRequestEventsItemIdentifiers {
    ///                         ..Default::default()
    ///                     },
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_batch_events(
        &self,
        request: &CreateBatchEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchAcceptedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "events/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
