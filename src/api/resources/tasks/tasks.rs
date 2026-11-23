use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of CRM tasks with optional filtering by task type, status, date range, assignee, and linked entities (contacts, deals, companies). Results are sorted by creation date in descending order by default, with a default limit of 50 tasks per page.
    ///
    /// # Arguments
    ///
    /// * `filter_type` - Filter by task type (ID)
    /// * `filter_status` - Filter by task status
    /// * `filter_date` - Filter by date
    /// * `filter_assign_to` - Filter by the "assignTo" ID. You can utilize account emails for the "assignTo" attribute.
    /// * `filter_contacts` - Filter by contact ids
    /// * `filter_deals` - Filter by deals ids
    /// * `filter_companies` - Filter by companies ids
    /// * `date_from` - dateFrom to date range filter type (timestamp in milliseconds)
    /// * `date_to` - dateTo to date range filter type (timestamp in milliseconds)
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
    ///         .tasks
    ///         .get_all_tasks(
    ///             &GetAllTasksQueryRequest {
    ///                 sort_by: Some("name".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_all_tasks(
        &self,
        request: &GetAllTasksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCrmTasksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "crm/tasks",
                None,
                QueryBuilder::new()
                    .string("filter[type]", request.filter_type.clone())
                    .serialize("filter[status]", request.filter_status.clone())
                    .serialize("filter[date]", request.filter_date.clone())
                    .string("filter[assignTo]", request.filter_assign_to.clone())
                    .string("filter[contacts]", request.filter_contacts.clone())
                    .string("filter[deals]", request.filter_deals.clone())
                    .string("filter[companies]", request.filter_companies.clone())
                    .int("dateFrom", request.date_from.clone())
                    .int("dateTo", request.date_to.clone())
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .serialize("sort", request.sort.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new CRM task with the specified name, type, due date, and optional associations to contacts, companies, or deals. A task requires a name, task type ID, and due date at minimum. You can also set a duration, notes, a reminder, and assign the task to a specific user.
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
    ///         .tasks
    ///         .create_a_task(
    ///             &PostCrmTasksRequest {
    ///                 date: DateTime::parse_from_rfc3339("2021-11-01T17:44:54Z").unwrap(),
    ///                 name: "Task: Connect with client".to_string(),
    ///                 task_type_id: "61a5cd07ca1347c82306ad09".to_string(),
    ///                 assign_to_id: None,
    ///                 companies_ids: None,
    ///                 contacts_ids: None,
    ///                 deals_ids: None,
    ///                 done: None,
    ///                 duration: None,
    ///                 notes: None,
    ///                 reminder: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_a_task(
        &self,
        request: &PostCrmTasksRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCrmTasksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "crm/tasks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the full details of a single CRM task by its identifier. The response includes the task''s name, type, status, due date, duration, notes, assignee, reminder settings, and linked contacts, companies, or deals.
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
    ///     client.tasks.get_a_task(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_a_task(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("crm/tasks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a CRM task by its identifier. This removes the task and cancels any associated reminders. The requesting user must be the task assignee or have manage permission on tasks.
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
    ///     client.tasks.delete_a_task(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete_a_task(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("crm/tasks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing CRM task''s properties such as name, type, due date, status, duration, notes, assignee, reminder, or linked entities. Only the fields provided in the request body will be updated; omitted fields remain unchanged.
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
    ///         .tasks
    ///         .update_a_task(
    ///             &"id".to_string(),
    ///             &PatchCrmTasksIdRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_a_task(
        &self,
        id: &str,
        request: &PatchCrmTasksIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("crm/tasks/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the list of all available task types, such as Email, Call, Meeting, Todo, Lunch, Deadline, and LinkedIn. If no task types exist yet, the default set is automatically created and returned. Use the task type ID when creating or updating tasks.
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
    ///     client.tasks.get_all_task_types(None).await;
    /// }
    /// ```
    pub async fn get_all_task_types(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetCrmTasktypesResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "crm/tasktypes", None, None, options)
            .await
    }
}
