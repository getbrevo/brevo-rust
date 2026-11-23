use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ProcessClient {
    pub http_client: HttpClient,
}

impl ProcessClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves a list of background processes from your Brevo account with filtering and pagination.
    ///
    /// **Use this to:**
    /// - Monitor background process activity and status
    /// - Track long-running operations and tasks
    /// - Find process IDs for detailed status checking
    /// - Review process history and performance
    /// - Identify failed or stuck processes for troubleshooting
    ///
    /// **Key information returned:**
    /// - Process details (ID, name, status)
    /// - Export download URLs for completed export processes
    /// - Import details with CSV report URLs for completed import processes
    /// - Total count of processes for pagination
    ///
    /// **Important considerations:**
    /// - Background processes handle long-running operations like imports and exports
    /// - Process status indicates current state (queued, processing, completed)
    /// - Export processes provide download URLs when completed
    /// - Import processes provide CSV report URLs with details about problematic records
    /// - Use pagination for accounts with many historical processes
    /// - Sort options available for creation order (ascending or descending)
    /// - Default limit is 10 results per page, maximum is 50
    ///
    /// # Arguments
    ///
    /// * `limit` - Number limitation for the result returned
    /// * `offset` - Beginning point in the list to retrieve from.
    /// * `sort` - Sort the results in the ascending/descending order of record creation.
    /// Default order is **descending** if `sort` is not passed
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
    ///         .process
    ///         .get_processes(
    ///             &GetProcessesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_processes(
        &self,
        request: &GetProcessesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetProcessesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "processes",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves detailed information about a specific background process.
    ///
    /// **Use this to:**
    /// - Get detailed status of a specific process
    /// - Monitor process progress and completion
    /// - Download results from completed export processes
    /// - Check error details for failed processes
    /// - Track process execution times
    ///
    /// **Key information returned:**
    /// - Complete process details (ID, name, status)
    /// - Download URLs for completed export processes
    /// - Import details with CSV report URLs for completed import processes
    ///
    /// **Important considerations:**
    /// - Process ID must exist in your account and not be deleted
    /// - Completed export processes include download URLs
    /// - Completed import processes include CSV report URLs with details about problematic records
    /// - Different process types return different result structures
    ///
    /// # Arguments
    ///
    /// * `process_id` - Id of the process
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
    ///     client.process.get_process(1000000, None).await;
    /// }
    /// ```
    pub async fn get_process(
        &self,
        process_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetProcessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("processes/{}", process_id),
                None,
                None,
                options,
            )
            .await
    }
}
