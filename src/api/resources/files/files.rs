use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FilesClient {
    pub http_client: HttpClient,
}

impl FilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of CRM files with optional filtering by entity type, entity IDs, and date range. Results are sorted by creation date in descending order by default, with a default limit of 50 files per page.
    ///
    /// # Arguments
    ///
    /// * `entity` - Filter by file entity type
    /// * `entity_ids` - Filter by file entity IDs
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
    ///         .files
    ///         .get_all_files(
    ///             &GetAllFilesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_all_files(
        &self,
        request: &GetAllFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<FileData>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "crm/files",
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

    /// Upload a file and associate it with a contact, company, or deal. The file must be sent as multipart form data with a maximum size of 10 MB. You can optionally link the file to a specific entity by providing the corresponding entity ID.
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
    ///         .files
    ///         .upload_a_file(
    ///             &UploadAFileRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 company_id: None,
    ///                 contact_id: None,
    ///                 deal_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload_a_file(
        &self,
        request: &UploadAFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<FileData, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "crm/files",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Get a temporary download URL for a CRM file by its identifier. The returned URL is valid for 5 minutes only and provides direct access to the file content.
    ///
    /// # Arguments
    ///
    /// * `id` - File id to download.
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
    ///     client.files.download_a_file(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn download_a_file(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetCrmFilesIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("crm/files/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a CRM file by its identifier. This removes the file from storage and unlinks it from any associated contacts, companies, or deals.
    ///
    /// # Arguments
    ///
    /// * `id` - File id to delete.
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
    ///     client.files.delete_a_file(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_a_file(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("crm/files/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve the metadata and details of a specific CRM file by its identifier. This returns information such as the file name, size, type, creation date, and associated entities, but does not include the file content itself.
    ///
    /// # Arguments
    ///
    /// * `id` - File id to get file data.
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
    ///     client.files.get_file_details(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_file_details(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<FileData, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("crm/files/{}/data", id),
                None,
                None,
                options,
            )
            .await
    }
}
