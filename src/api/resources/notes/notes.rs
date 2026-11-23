use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct NotesClient {
    pub http_client: HttpClient,
}

impl NotesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of CRM notes with optional filtering by entity type, entity IDs, and date range. Results are sorted by creation date in descending order by default, with a default limit of 50 notes per page.
    ///
    /// # Arguments
    ///
    /// * `entity` - Filter by note entity type
    /// * `entity_ids` - Filter by note entity IDs
    /// * `date_from` - dateFrom to date range filter type (timestamp in milliseconds)
    /// * `date_to` - dateTo to date range filter type (timestamp in milliseconds)
    /// * `offset` - Index of the first document of the page
    /// * `limit` - Number of documents per page
    /// * `sort` - Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
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
    ///         .notes
    ///         .get_all_notes(
    ///             &GetAllNotesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_all_notes(
        &self,
        request: &GetAllNotesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Note>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "crm/notes",
                None,
                QueryBuilder::new()
                    .serialize("entity", request.entity.clone())
                    .string("entityIds", request.entity_ids.clone())
                    .int("dateFrom", request.date_from.clone())
                    .int("dateTo", request.date_to.clone())
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new CRM note and optionally associate it with contacts, companies, or deals. The note text content is required, and you can link the note to multiple entities simultaneously during creation.
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
    ///     client.notes.create_a_note(&NoteData {
    ///         text: "<p>Meeting notes: <b>Action item</b> - visit <a href=\"https://www.brevo.com/\">Brevo</a> for details.</p>".to_string(),
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create_a_note(
        &self,
        request: &NoteData,
        options: Option<RequestOptions>,
    ) -> Result<PostCrmNotesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "crm/notes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the full details of a single CRM note by its identifier. The response includes the note''s text content, creation date, author, and any associated contacts, companies, or deals.
    ///
    /// # Arguments
    ///
    /// * `id` - Note ID to get
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
    ///     client.notes.get_a_note(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_a_note(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Note, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("crm/notes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a CRM note by its identifier. This removes the note and unlinks it from any associated contacts, companies, or deals.
    ///
    /// # Arguments
    ///
    /// * `id` - Note ID to delete
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
    ///     client.notes.delete_a_note(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_a_note(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("crm/notes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing CRM note''s text content and its associations with contacts, companies, or deals. You can modify the note text, change the pinned status, or update the linked entities.
    ///
    /// # Arguments
    ///
    /// * `id` - Note ID to update
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
    ///     client.notes.update_a_note(&"id".to_string(), &NoteData {
    ///         text: "<p>Meeting notes: <b>Action item</b> - visit <a href=\"https://www.brevo.com/\">Brevo</a> for details.</p>".to_string(),
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn update_a_note(
        &self,
        id: &str,
        request: &NoteData,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("crm/notes/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
