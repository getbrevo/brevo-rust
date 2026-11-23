use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ContactsClient {
    pub http_client: HttpClient,
}

impl ContactsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all contacts from your Brevo account with support for pagination, filtering, and sorting. Results default to 50 contacts per page (maximum 1000) sorted in descending order of creation, and can be filtered by modification date, creation date, contact IDs (up to 20), list IDs, segment ID, or contact attributes using the equals operator. Note that either listIds or segmentId can be passed but not both simultaneously.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document of the page
    /// * `modified_since` - Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `created_since` - Filter (urlencoded) the contacts created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    /// * `ids` - Filter by a list of contact IDs. You can pass a **maximum of 20 IDs**. All elements must be integers.
    /// * `segment_id` - Id of the segment. **Either listIds or segmentId can be passed.** Must be a positive integer (minimum value of 1).
    /// * `list_ids` - Ids of the list. **Either listIds or segmentId can be passed.**
    /// * `filter` - Filter the contacts on the basis of attributes. **Allowed operator: equals. For multiple-choice options, the filter will apply an AND condition between the options. For category attributes, the filter will work with both id and value. (e.g. filter=equals(FIRSTNAME,"Antoine"), filter=equals(B1, true), filter=equals(DOB, "1989-11-23"), filter=equals(GENDER, "1"), filter=equals(GENDER, "MALE"), filter=equals(COUNTRY,"USA, INDIA")**
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
    ///         .contacts
    ///         .get_contacts(
    ///             &GetContactsQueryRequest {
    ///                 limit: None,
    ///                 offset: None,
    ///                 modified_since: None,
    ///                 created_since: None,
    ///                 sort: None,
    ///                 ids: vec![],
    ///                 segment_id: None,
    ///                 list_ids: vec![],
    ///                 filter: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_contacts(
        &self,
        request: &GetContactsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetContacts, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "contacts",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("modifiedSince", request.modified_since.clone())
                    .string("createdSince", request.created_since.clone())
                    .serialize("sort", request.sort.clone())
                    .int_array("ids", request.ids.clone())
                    .int("segmentId", request.segment_id.clone())
                    .int_array("listIds", request.list_ids.clone())
                    .string("filter", request.filter.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Note>Follow this format when passing a "SMS" phone number as an attribute.
    /// Accepted Number Formats 91xxxxxxxxxx +91xxxxxxxxxx 0091xxxxxxxxxx</Note>
    /// Creates new contacts on Brevo. Contacts can be created by passing either - <br><br> 1. email address of the contact (email_id),  <br> 2. phone number of the contact (to be passed as "SMS" field in "attributes" along with proper country code), For example- {"SMS":"+91xxxxxxxxxx"} or {"SMS":"0091xxxxxxxxxx"} <br> 3. ext_id <br>
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
    ///         .contacts
    ///         .create_contact(
    ///             &CreateContactRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_contact(
        &self,
        request: &CreateContactRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<CreateContactResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve all contact attributes defined in your Brevo account, grouped by category (normal, transactional, category, calculated, global). Each attribute includes its name, type, and category, along with enumeration values for category-type attributes and options for multiple-choice-type attributes.
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
    ///     client.contacts.get_attributes(None).await;
    /// }
    /// ```
    pub async fn get_attributes(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetAttributesResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "contacts/attributes", None, None, options)
            .await
    }

    /// Create a new contact attribute under the specified category and name. The required body properties depend on the category: use "type" for normal, transactional, or category attributes; use "value" for calculated or global attributes; use "enumeration" for category attributes; and use "multiCategoryOptions" for normal multiple-choice attributes. None of the category or multicategory option values can exceed 200 characters.
    ///
    /// # Arguments
    ///
    /// * `attribute_category` - Category of the attribute
    /// * `attribute_name` - Name of the attribute
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
    ///         .contacts
    ///         .create_attribute(
    ///             &CreateAttributeRequestAttributeCategory::Normal,
    ///             &"attributeName".to_string(),
    ///             &CreateAttributeRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_attribute(
        &self,
        attribute_category: &CreateAttributeRequestAttributeCategory,
        attribute_name: &str,
        request: &CreateAttributeRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "contacts/attributes/{}/{}",
                    attribute_category, attribute_name
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update an existing contact attribute identified by its category and name. For category-type attributes, you can update the enumeration values; for calculated or global attributes, update the computed value formula; and for normal multiple-choice attributes, update the multicategory options. None of the category or multicategory option values can exceed 200 characters.
    ///
    /// # Arguments
    ///
    /// * `attribute_category` - Category of the attribute
    /// * `attribute_name` - Name of the existing attribute
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
    ///         .contacts
    ///         .update_attribute(
    ///             &UpdateAttributeRequestAttributeCategory::Category,
    ///             &"attributeName".to_string(),
    ///             &UpdateAttributeRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_attribute(
        &self,
        attribute_category: &UpdateAttributeRequestAttributeCategory,
        attribute_name: &str,
        request: &UpdateAttributeRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "contacts/attributes/{}/{}",
                    attribute_category, attribute_name
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Permanently delete an existing contact attribute by its category and name. The attribute must exist in the specified category (normal, transactional, category, calculated, or global), otherwise a 404 error is returned.
    ///
    /// # Arguments
    ///
    /// * `attribute_category` - Category of the attribute
    /// * `attribute_name` - Name of the existing attribute
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
    ///         .contacts
    ///         .delete_attribute(
    ///             &DeleteAttributeRequestAttributeCategory::Normal,
    ///             &"attributeName".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_attribute(
        &self,
        attribute_category: &DeleteAttributeRequestAttributeCategory,
        attribute_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "contacts/attributes/{}/{}",
                    attribute_category, attribute_name
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a specific option from an existing multiple-choice contact attribute. The attribute type must be "multiple-choice", and both the attribute name and the option to delete must already exist in your account.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - Type of the attribute
    /// * `multiple_choice_attribute` - Name of the existing multiple-choice attribute
    /// * `multiple_choice_attribute_option` - Name of the existing multiple-choice attribute option that you want to delete
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
    ///         .contacts
    ///         .delete_multi_attribute_options(
    ///             &"multiple-choice".to_string(),
    ///             &"multipleChoiceAttribute".to_string(),
    ///             &"multipleChoiceAttributeOption".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_multi_attribute_options(
        &self,
        attribute_type: &str,
        multiple_choice_attribute: &str,
        multiple_choice_attribute_option: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "contacts/attributes/{}/{}/{}",
                    attribute_type, multiple_choice_attribute, multiple_choice_attribute_option
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update multiple contacts in a single API call by passing an array of contact objects. Each contact in the array must be identified by one of: email, id, or sms (only one identifier per contact). You can update attributes, blacklist status, list memberships, ext_id, and transactional email forbidden senders for each contact in the batch.
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
    ///         .contacts
    ///         .update_batch_contacts(
    ///             &UpdateBatchContactsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_batch_contacts(
        &self,
        request: &UpdateBatchContactsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts/batch",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note title="How to use attributes param?">attributes param in this endpoint is an object containing key-value pairs where values can be either a string, integer, array, or boolean. You can create key-value pairs with these four datatypes. When a value is an array, it should be an array of strings.</Note>
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
    ///         .contacts
    ///         .create_doi_contact(
    ///             &CreateDoiContactRequest {
    ///                 email: "elly@example.com".to_string(),
    ///                 include_list_ids: vec![36],
    ///                 redirection_url: "http://requestb.in/173lyyx1".to_string(),
    ///                 template_id: 2,
    ///                 attributes: None,
    ///                 contact_pixel_tracking_consent: None,
    ///                 exclude_list_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_doi_contact(
        &self,
        request: &CreateDoiContactRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts/doubleOptinConfirmation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// It returns the background process ID which on completion calls the notify URL that you have set in the input. File will be available in csv.
    ///
    /// When no contact matches the filter, no export process is created: the endpoint answers `200` with an empty object instead of `202` with a processId. A recurring export should treat that as "nothing to do" rather than as a failure, and must not assume a processId is always present.
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
    ///         .contacts
    ///         .request_contact_export(
    ///             &RequestContactExportRequest {
    ///                 custom_contact_filter: RequestContactExportRequestCustomContactFilter {
    ///                     ..Default::default()
    ///                 },
    ///                 disable_notification: None,
    ///                 export_attributes: None,
    ///                 export_mandatory_attributes: None,
    ///                 export_metadata: None,
    ///                 export_date_in_utc: None,
    ///                 export_subscription_status: None,
    ///                 notify_url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn request_contact_export(
        &self,
        request: &RequestContactExportRequest,
        options: Option<RequestOptions>,
    ) -> Result<RequestContactExportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts/export",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note>Ongoing changes for this endpoint
    /// We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
    /// These are non breaking changes.
    /// The default value for the attributes will be 0.
    /// The uniqueSubscribers field is deprecated</Note>
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document of the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
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
    ///         .contacts
    ///         .get_folders(
    ///             &GetFoldersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_folders(
        &self,
        request: &GetFoldersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetFoldersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "contacts/folders",
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

    /// Create a new folder to organize your contact lists. Folders serve as containers for grouping related lists together. The folder name is required and must be provided in the request body.
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
    ///         .contacts
    ///         .create_folder(
    ///             &CreateUpdateFolder {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_folder(
        &self,
        request: &CreateUpdateFolder,
        options: Option<RequestOptions>,
    ) -> Result<CreateFolderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts/folders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note>Ongoing changes for this endpoint.
    /// We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
    /// These are non breaking changes. The default value for the attributes will be 0.</Note>
    ///
    /// # Arguments
    ///
    /// * `folder_id` - id of the folder
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
    ///     client.contacts.get_folder(1000000, None).await;
    /// }
    /// ```
    pub async fn get_folder(
        &self,
        folder_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetFolder, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("contacts/folders/{}", folder_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the name of an existing folder identified by its ID. The new folder name must be provided in the request body. Returns a 404 error if the folder ID does not exist.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - Id of the folder
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
    ///         .contacts
    ///         .update_folder(
    ///             1000000,
    ///             &CreateUpdateFolder {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_folder(
        &self,
        folder_id: i64,
        request: &CreateUpdateFolder,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("contacts/folders/{}", folder_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Permanently delete a folder identified by its ID. Deleting a folder will also delete all the contact lists contained within it. This action cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - Id of the folder
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
    ///     client.contacts.delete_folder(1000000, None).await;
    /// }
    /// ```
    pub async fn delete_folder(
        &self,
        folder_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("contacts/folders/{}", folder_id),
                None,
                None,
                options,
            )
            .await
    }

    /// <Note>Ongoing changes for this endpoint.
    /// We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
    /// These are non breaking changes. The default value for the attributes will be 0.</Note>
    ///
    /// # Arguments
    ///
    /// * `folder_id` - Id of the folder
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document of the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
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
    ///         .contacts
    ///         .get_folder_lists(
    ///             1000000,
    ///             &GetFolderListsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_folder_lists(
        &self,
        folder_id: i64,
        request: &GetFolderListsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetFolderListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("contacts/folders/{}/lists", folder_id),
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

    /// It returns the background process ID which on completion calls the notify URL that you have set in the input. **Note**: - Any contact attribute that doesn't exist in your account will be ignored at import end.
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
    ///         .contacts
    ///         .import_contacts(
    ///             &ImportContactsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn import_contacts(
        &self,
        request: &ImportContactsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ImportContactsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts/import",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// <Note>Ongoing changes for this endpoint.
    /// We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
    /// These are non breaking changes. The default value for the attributes will be 0.</Note>
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document of the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
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
    ///         .contacts
    ///         .get_lists(
    ///             &GetListsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_lists(
        &self,
        request: &GetListsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetListsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "contacts/lists",
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

    /// Create a new contact list inside a specified folder. Both the list name and the parent folder ID are required. The newly created list will be empty and ready to receive contacts via the add contacts endpoint.
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
    ///         .contacts
    ///         .create_list(
    ///             &CreateListRequest {
    ///                 folder_id: 2,
    ///                 name: "Magento Customer - ES".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_list(
        &self,
        request: &CreateListRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "contacts/lists",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the details of a specific contact list by its ID, including its name, folder ID, creation date, subscriber counts, and campaign statistics. You can optionally filter campaign statistics by providing startDate and endDate parameters (both must be used together in YYYY-MM-DD format).
    ///
    /// # Arguments
    ///
    /// * `list_id` - Id of the list
    /// * `start_date` - **Mandatory if endDate is used**. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to aggregate the sent email campaigns for a specific list id. **Prefer to pass your timezone in date-time format for accurate result**
    /// * `end_date` - **Mandatory if startDate is used**. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to aggregate the sent email campaigns for a specific list id. **Prefer to pass your timezone in date-time format for accurate result**
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
    ///         .contacts
    ///         .get_list(
    ///             1000000,
    ///             &GetListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_list(
        &self,
        list_id: i64,
        request: &GetListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("contacts/lists/{}", list_id),
                None,
                QueryBuilder::new()
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update an existing contact list identified by its ID. You can update the list name, move it to a different folder by providing a new folderId, or both. Only one of the two parameters (name, folderId) needs to be provided per request.
    ///
    /// # Arguments
    ///
    /// * `list_id` - Id of the list
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
    ///         .contacts
    ///         .update_list(
    ///             1000000,
    ///             &UpdateListRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_list(
        &self,
        list_id: i64,
        request: &UpdateListRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("contacts/lists/{}", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Permanently delete a contact list identified by its ID. The contacts in the list are not deleted; they are only removed from this list. Returns a 404 error if the list ID does not exist.
    ///
    /// # Arguments
    ///
    /// * `list_id` - Id of the list
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
    ///     client.contacts.delete_list(1000000, None).await;
    /// }
    /// ```
    pub async fn delete_list(
        &self,
        list_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("contacts/lists/{}", list_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all contacts belonging to a specific list, identified by its list ID. Results are paginated with a default of 50 contacts per page (maximum 500) and sorted in descending order of creation. You can optionally filter contacts by their modification date using the modifiedSince parameter.
    ///
    /// # Arguments
    ///
    /// * `list_id` - Id of the list
    /// * `modified_since` - Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document of the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
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
    ///         .contacts
    ///         .get_contacts_from_list(
    ///             1000000,
    ///             &GetContactsFromListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_contacts_from_list(
        &self,
        list_id: i64,
        request: &GetContactsFromListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetContacts, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("contacts/lists/{}/contacts", list_id),
                None,
                QueryBuilder::new()
                    .string("modifiedSince", request.modified_since.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

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
    ///         .contacts
    ///         .add_contact_to_list(
    ///             1000000,
    ///             &AddContactToListRequestBody::AddContactToListRequestBodyEmails(
    ///                 AddContactToListRequestBodyEmails {
    ///                     emails: Some(vec![
    ///                         "jeff32@example.com".to_string(),
    ///                         "jim56@example.com".to_string(),
    ///                     ]),
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_contact_to_list(
        &self,
        list_id: i64,
        request: &AddContactToListRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<PostContactInfo, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("contacts/lists/{}/contacts/add", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

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
    ///         .contacts
    ///         .remove_contact_from_list(
    ///             1000000,
    ///             &RemoveContactFromListRequestBody::RemoveContactFromListRequestBodyEmails(
    ///                 RemoveContactFromListRequestBodyEmails {
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove_contact_from_list(
        &self,
        list_id: i64,
        request: &RemoveContactFromListRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<PostContactInfo, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("contacts/lists/{}/contacts/remove", list_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve all contact segments defined in your Brevo account with support for pagination and sorting. Results default to 10 segments per page (maximum 50) sorted in descending order of creation. Each segment includes its ID, name, category name, and last update timestamp.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of documents per page
    /// * `offset` - Index of the first document of the page
    /// * `sort` - Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
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
    ///         .contacts
    ///         .get_segments(
    ///             &GetSegmentsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_segments(
        &self,
        request: &GetSegmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSegmentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "contacts/segments",
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

    /// <Note>Follow this format when passing a "SMS" phone number as an attribute.
    /// Accepted Number Formats 91xxxxxxxxxx +91xxxxxxxxxx 0091xxxxxxxxxx</Note>
    /// There are 2 ways to get a contact <br><br> Option 1- https://api.brevo.com/v3/contacts/{identifier} <br><br> Option 2- https://api.brevo.com/v3/contacts/{identifier}?identifierType={} <br> <br> Option 1 only works if identifierType is email_id (for EMAIL), phone_id (for SMS) or contact_id (for ID of the contact),where you can directly pass the value of EMAIL, SMS and ID of the contact.   <br><br> Option 2 works for all identifierType, use email_id for EMAIL attribute, phone_id for SMS attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute <br><br>Along with the contact details, this endpoint will show the statistics of contact for the recent 90 days by default. To fetch the earlier statistics, please use Get contact campaign stats ``https://developers.brevo.com/reference/contacts-7#getcontactstats`` endpoint with the appropriate date ranges.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Email (urlencoded) OR ID of the contact OR its SMS attribute value OR EXT_ID attribute (urlencoded)
    /// * `identifier_type` - email_id for Email, phone_id for SMS attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
    /// * `start_date` - **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be lower than equal to endDate. Must not be greater than the current date.
    /// * `end_date` - **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be greater than equal to startDate. Must not be greater than the current date.
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
    ///         .contacts
    ///         .get_contact_info(
    ///             &GetContactInfoRequestIdentifier::String("identifier".to_string()),
    ///             &GetContactInfoQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_contact_info(
        &self,
        identifier: &GetContactInfoRequestIdentifier,
        request: &GetContactInfoQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetContactInfoResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("contacts/{}", identifier),
                None,
                QueryBuilder::new()
                    .serialize("identifierType", request.identifier_type.clone())
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Note>Follow this format when passing a "SMS" phone number as an attribute.
    /// Accepted Number Formats 91xxxxxxxxxx +91xxxxxxxxxx 0091xxxxxxxxxx <br><br> If a blocklisted contact's email address is updated, it is going to remove that blocklisting from the contact and they will be resubscribed.</Note>
    /// There are 2 ways to update a contact <br><br> Option 1- https://api.brevo.com/v3/contacts/{identifier} <br><br> Option 2- https://api.brevo.com/v3/contacts/{identifier}?identifierType={} <br> <br> Option 1 only works if identifierType is email_id (for EMAIL) or contact_id (for ID of the contact),where you can directly pass the value of EMAIL and ID of the contact.   <br><br> Option 2 works for all identifierType, use email_id for EMAIL attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE attribute
    ///
    /// # Arguments
    ///
    /// * `identifier` - Email (urlencoded) OR ID of the contact OR EXT_ID attribute (urlencoded) OR its SMS attribute value OR its WHATSAPP attribute value OR its LANDLINE_NUMBER attribute value
    /// * `identifier_type` - email_id for Email, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
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
    ///         .contacts
    ///         .update_contact(
    ///             &UpdateContactRequestIdentifier::String("identifier".to_string()),
    ///             &UpdateContactRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_contact(
        &self,
        identifier: &UpdateContactRequestIdentifier,
        request: &UpdateContactRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("contacts/{}", identifier),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("identifierType", request.identifier_type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// There are 2 ways to delete a contact <br><br> Option 1- https://api.brevo.com/v3/contacts/{identifier} <br><br> Option 2- https://api.brevo.com/v3/contacts/{identifier}?identifierType={} <br> <br> Option 1 only works if identifierType is email_id (for EMAIL) or contact_id (for ID of the contact),where you can directly pass the value of EMAIL and ID of the contact.   <br><br> Option 2 works for all identifierType, use email_id for EMAIL attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Email (urlencoded) OR ID of the contact OR EXT_ID attribute (urlencoded)
    /// * `identifier_type` - email_id for Email, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
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
    ///         .contacts
    ///         .delete_contact(
    ///             &DeleteContactRequestIdentifier::String("identifier".to_string()),
    ///             &DeleteContactQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_contact(
        &self,
        identifier: &DeleteContactRequestIdentifier,
        request: &DeleteContactQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("contacts/{}", identifier),
                None,
                QueryBuilder::new()
                    .serialize("identifierType", request.identifier_type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve email campaign statistics for a specific contact identified by email address or numeric ID. Statistics include messages sent, opens, clicks, hard/soft bounces, deliveries, unsubscriptions, complaints, and transactional attributes. By default, data covers the last 90 days; use startDate and endDate parameters (YYYY-MM-DD) to specify a custom range with a maximum span of 90 days.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Email (urlencoded) OR ID of the contact
    /// * `start_date` - **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be lower than equal to endDate. Must not be greater than the current date.
    /// * `end_date` - **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be greater than equal to startDate. Must not be greater than the current date. Maximum difference between startDate and endDate should not be greater than 90 days.
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
    ///         .contacts
    ///         .get_contact_stats(
    ///             &GetContactStatsRequestIdentifier::String("identifier".to_string()),
    ///             &GetContactStatsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_contact_stats(
        &self,
        identifier: &GetContactStatsRequestIdentifier,
        request: &GetContactStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetContactStatsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("contacts/{}/campaignStats", identifier),
                None,
                QueryBuilder::new()
                    .string("startDate", request.start_date.clone())
                    .string("endDate", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }
}
