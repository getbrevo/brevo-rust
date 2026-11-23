use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct MasterAccountClient {
    pub http_client: HttpClient,
}

impl MasterAccountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// This endpoint allows to create a group of sub-accounts
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
    ///         .master_account
    ///         .create_a_new_group_of_sub_accounts(
    ///             &PostCorporateGroupRequest {
    ///                 group_name: "My group".to_string(),
    ///                 sub_account_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_a_new_group_of_sub_accounts(
        &self,
        request: &PostCorporateGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCorporateGroupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corporate/group",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint allows you to remove a sub-organization from a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - Group id
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
    ///         .master_account
    ///         .delete_sub_account_from_group(
    ///             &"groupId".to_string(),
    ///             &PutCorporateGroupUnlinkGroupIdSubAccountsRequest {
    ///                 sub_account_ids: vec![423432, 234323, 87678],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_sub_account_from_group(
        &self,
        group_id: &str,
        request: &PutCorporateGroupUnlinkGroupIdSubAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("corporate/group/unlink/{}/subAccounts", group_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint allows you to retrieve a specific group’s information such as
    /// the list of sub-organizations and the user associated with the group.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the group of sub-organization
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
    ///         .master_account
    ///         .get_a_group_details(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_a_group_details(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetCorporateGroupIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("corporate/group/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// This endpoint allows to update a group of sub-accounts
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the group
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
    ///         .master_account
    ///         .update_a_group_of_sub_accounts(
    ///             &"id".to_string(),
    ///             &PutCorporateGroupIdRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_a_group_of_sub_accounts(
        &self,
        id: &str,
        request: &PutCorporateGroupIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("corporate/group/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint allows you to delete a group of sub-organizations. When a
    /// group is deleted, the sub-organizations are no longer part of this group.
    /// The users associated with the group are no longer associated with the group
    /// once deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the group
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
    ///         .master_account
    ///         .delete_a_group(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_a_group(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("corporate/group/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// This endpoint allows you to list all groups created on your Admin account.
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
    ///     client.master_account.get_sub_account_groups(None).await;
    /// }
    /// ```
    pub async fn get_sub_account_groups(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetSubAccountGroupsResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "corporate/groups", None, None, options)
            .await
    }

    /// This endpoint allows you to list all Admin users of your Admin account. You
    /// can filter users by type (active or pending) and paginate results using
    /// offset and limit.
    ///
    /// # Arguments
    ///
    /// * `type_` - User type (active | pending). This is required if offset is provided for
    /// limited result.
    /// * `offset` - Page number for the result set. This is optional, default value will be
    /// the 1st page.
    /// * `limit` - Number of users to be displayed on each page. This is optional, the
    /// default limit is 20, but max allowed limit is 100.
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
    ///         .master_account
    ///         .get_corporate_invited_users_list(
    ///             &GetCorporateInvitedUsersListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_corporate_invited_users_list(
        &self,
        request: &GetCorporateInvitedUsersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCorporateInvitedUsersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "corporate/invited/users",
                None,
                QueryBuilder::new()
                    .string("type", request.r#type.clone())
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This endpoint allows you to retrieve the list of active IPs on your Admin
    /// account
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
    ///     client.master_account.list_of_all_i_ps(None).await;
    /// }
    /// ```
    pub async fn list_of_all_i_ps(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetCorporateIpResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "corporate/ip", None, None, options)
            .await
    }

    /// This endpoint will provide the details of the master account.
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
    ///         .master_account
    ///         .get_the_details_of_requested_master_account(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_the_details_of_requested_master_account(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetCorporateMasterAccountResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "corporate/masterAccount", None, None, options)
            .await
    }

    /// This endpoint generates an SSO token to authenticate and access the admin
    /// account using the endpoint
    /// https://account-app.brevo.com/account/login/corporate/sso/[token], where
    /// [token] will be replaced by the actual token.
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
    ///         .master_account
    ///         .generate_sso_token_to_access_admin_account(
    ///             &PostCorporateSsoTokenRequest {
    ///                 email: "vipin+ent-user@brevo.com".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn generate_sso_token_to_access_admin_account(
        &self,
        request: &PostCorporateSsoTokenRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSsoToken, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corporate/ssoToken",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint will provide the list all the sub-accounts of the master
    /// account.
    ///
    /// # Arguments
    ///
    /// * `offset` - Index of the first sub-account in the page
    /// * `limit` - Number of sub-accounts to be displayed on each page
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
    ///         .master_account
    ///         .get_the_list_of_all_the_sub_accounts_of_the_master_account(
    ///             &GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest {
    ///                 offset: 1,
    ///                 limit: 1,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_the_list_of_all_the_sub_accounts_of_the_master_account(
        &self,
        request: &GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCorporateSubAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "corporate/subAccount",
                None,
                QueryBuilder::new()
                    .int("offset", request.offset.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// This endpoint will create a new sub-account under a master account
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
    ///         .master_account
    ///         .create_a_new_sub_account_under_a_master_account(
    ///             &PostCorporateSubAccountRequest {
    ///                 company_name: "Test Sub-account".to_string(),
    ///                 email: "test-sub@example.com".to_string(),
    ///                 group_ids: Some(vec![
    ///                     "5f8f8c3b5f56a02d4433b3a7".to_string(),
    ///                     "5f8f8c3b5f56a02d4433b3a8".to_string(),
    ///                 ]),
    ///                 language: Some(PostCorporateSubAccountRequestLanguage::Fr),
    ///                 timezone: Some("Europe/Paris".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_a_new_sub_account_under_a_master_account(
        &self,
        request: &PostCorporateSubAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCorporateSubAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corporate/subAccount",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint allows to associate an IP to sub-accounts
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
    ///         .master_account
    ///         .associate_an_ip_to_sub_accounts(
    ///             &PostCorporateSubAccountIpAssociateRequest {
    ///                 ids: vec![234322, 325553, 893432],
    ///                 ip: "103.11.32.88".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn associate_an_ip_to_sub_accounts(
        &self,
        request: &PostCorporateSubAccountIpAssociateRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corporate/subAccount/ip/associate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint allows to dissociate an IP from sub-accounts
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
    ///         .master_account
    ///         .dissociate_an_ip_to_sub_accounts(
    ///             &PutCorporateSubAccountIpDissociateRequest {
    ///                 ids: vec![234322, 325553, 893432],
    ///                 ip: "103.11.32.88".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn dissociate_an_ip_to_sub_accounts(
        &self,
        request: &PutCorporateSubAccountIpDissociateRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "corporate/subAccount/ip/dissociate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint will generate an API v3 key for a sub-account
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
    ///         .master_account
    ///         .create_an_api_key_for_a_sub_account(
    ///             &PostCorporateSubAccountKeyRequest {
    ///                 id: 3232323,
    ///                 name: "My Api Key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_an_api_key_for_a_sub_account(
        &self,
        request: &PostCorporateSubAccountKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostCorporateSubAccountKeyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corporate/subAccount/key",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint generates an sso token to authenticate and access a
    /// sub-account of the master using the account endpoint
    /// https://account-app.brevo.com/account/login/sub-account/sso/[token], where
    /// [token] will be replaced by the actual token.
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
    ///         .master_account
    ///         .generate_sso_token_to_access_sub_account(
    ///             &PostCorporateSubAccountSsoTokenRequest {
    ///                 id: 3232323,
    ///                 email: None,
    ///                 target: None,
    ///                 url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn generate_sso_token_to_access_sub_account(
        &self,
        request: &PostCorporateSubAccountSsoTokenRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSsoToken, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corporate/subAccount/ssoToken",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint will provide the details for the specified sub-account company
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the sub-account organization
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
    ///         .master_account
    ///         .get_sub_account_details(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_sub_account_details(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetCorporateSubAccountIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("corporate/subAccount/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a sub-account from the corporate master account. Once deleted, all data associated with the sub-account organization is removed and cannot be recovered, so ensure the sub-account is no longer needed before proceeding.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the sub-account organization to be deleted
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
    ///         .master_account
    ///         .delete_a_sub_account(1000000, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_a_sub_account(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("corporate/subAccount/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// API endpoint for the Corporate owner to enable/disable applications on the
    /// sub-account
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the sub-account organization (mandatory)
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
    ///         .master_account
    ///         .enable_disable_sub_account_application_s(
    ///             1000000,
    ///             &PutCorporateSubAccountIdApplicationsToggleRequest {
    ///                 landing_pages: Some(true),
    ///                 meetings: Some(true),
    ///                 sms_campaigns: Some(false),
    ///                 web_push: Some(false),
    ///                 whatsapp: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn enable_disable_sub_account_application_s(
        &self,
        id: i64,
        request: &PutCorporateSubAccountIdApplicationsToggleRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("corporate/subAccount/{}/applications/toggle", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint will update the sub-account plan. On the Corporate solution
    /// new version v2, you can set an unlimited number of credits in your
    /// sub-organization. Please pass the value “-1" to set the consumable in
    /// unlimited mode.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the sub-account organization
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
    ///         .master_account
    ///         .update_sub_account_plan(
    ///             1000000,
    ///             &PutCorporateSubAccountIdPlanRequest {
    ///                 credits: Some(PutCorporateSubAccountIdPlanRequestCredits {
    ///                     email: Some(5000),
    ///                     external_feeds: Some(1),
    ///                     sms: Some(2000.0),
    ///                     whatsapp: Some(100.0),
    ///                     wp_subscribers: Some(-1),
    ///                     ..Default::default()
    ///                 }),
    ///                 features: Some(PutCorporateSubAccountIdPlanRequestFeatures {
    ///                     inbox: Some(10),
    ///                     landing_page: Some(20),
    ///                     sales_users: Some(6),
    ///                     users: Some(15),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_sub_account_plan(
        &self,
        id: i64,
        request: &PutCorporateSubAccountIdPlanRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("corporate/subAccount/{}/plan", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint will update multiple sub-accounts plan. On the Corporate
    /// solution new version v2, you can set an unlimited number of credits in your
    /// sub-organization. Please pass the value “-1" to set the consumable in
    /// unlimited mode.
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
    ///         .master_account
    ///         .update_sub_accounts_plan(
    ///             &PutCorporateSubAccountsPlanRequest {
    ///                 credits: Some(PutCorporateSubAccountsPlanRequestCredits {
    ///                     email: Some(5000),
    ///                     external_feeds: Some(1),
    ///                     sms: Some(2000.0),
    ///                     whatsapp: Some(100.0),
    ///                     wp_subscribers: Some(-1),
    ///                     ..Default::default()
    ///                 }),
    ///                 features: Some(PutCorporateSubAccountsPlanRequestFeatures {
    ///                     landing_page: Some(20),
    ///                     sales_users: Some(6),
    ///                     users: Some(15),
    ///                     ..Default::default()
    ///                 }),
    ///                 sub_account_ids: Some(vec![4534345, 987893, 876785]),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_sub_accounts_plan(
        &self,
        request: &PutCorporateSubAccountsPlanRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "corporate/subAccounts/plan",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// `This endpoint allows you to invite a member to manage the Admin account
    /// Features and their respective permissions are as below: - `my_plan`:
    /// - "all"
    /// - `api`:
    /// - "none"
    /// - `user_management`:
    /// - "all"
    /// - `app_management` | Not available in ENTv2:
    /// - "all"
    /// - `sub_organization_groups`
    /// - "create"
    /// - "edit_delete"
    /// - `create_sub_organizations`
    /// - "all"
    /// - `manage_sub_organizations`
    /// - "all"
    /// - `analytics`
    /// - "download_data"
    /// - "create_alerts"
    /// - "my_looks"
    /// - "explore_create"
    /// - `security`
    /// - "all"
    /// **Note**: - If `all_features_access: false` then only privileges are
    /// required otherwise if `true` then it's assumed that all permissions will be
    /// there for the invited admin user.
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
    ///         .master_account
    ///         .invite_admin_user(
    ///             &InviteAdminUserRequest {
    ///                 all_features_access: true,
    ///                 email: "inviteuser@example.com".to_string(),
    ///                 privileges: vec![InviteAdminUserRequestPrivilegesItem {
    ///                     ..Default::default()
    ///                 }],
    ///                 group_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn invite_admin_user(
        &self,
        request: &InviteAdminUserRequest,
        options: Option<RequestOptions>,
    ) -> Result<InviteAdminUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corporate/user/invitation/send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// This endpoint will allow the user to:
    /// - Resend an admin user invitation
    /// - Cancel an admin user invitation
    ///
    /// # Arguments
    ///
    /// * `action` - Action to be performed (cancel / resend)
    /// * `email` - Email address of the recipient
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
    ///         .master_account
    ///         .resend_cancel_admin_user_invitation(
    ///             &PutCorporateUserInvitationActionEmailRequestAction::Resend,
    ///             &"email".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resend_cancel_admin_user_invitation(
        &self,
        action: &PutCorporateUserInvitationActionEmailRequestAction,
        email: &str,
        options: Option<RequestOptions>,
    ) -> Result<PutCorporateUserInvitationActionEmailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("corporate/user/invitation/{}/{}", action, email),
                None,
                None,
                options,
            )
            .await
    }

    /// This endpoint allows to revoke/remove an invited member of your Admin
    /// account
    ///
    /// # Arguments
    ///
    /// * `email` - Email of the invited user
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
    ///         .master_account
    ///         .revoke_an_admin_user(&"email".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn revoke_an_admin_user(
        &self,
        email: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("corporate/user/revoke/{}", email),
                None,
                None,
                options,
            )
            .await
    }

    /// This endpoint will provide the list of admin user permissions
    ///
    /// # Arguments
    ///
    /// * `email` - Email of the invited user.
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
    ///         .master_account
    ///         .get_corporate_user_permission(&"email".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_corporate_user_permission(
        &self,
        email: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetCorporateUserPermissionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("corporate/user/{}/permissions", email),
                None,
                None,
                options,
            )
            .await
    }

    /// This endpoint will allow you to change the permissions of Admin users of
    /// your Admin account
    ///
    /// # Arguments
    ///
    /// * `email` - Email address of Admin user
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
    ///     client.master_account.change_admin_user_permissions(&"email".to_string(), &PutCorporateUserEmailPermissionsRequest {
    ///         all_features_access: false,
    ///         privileges: vec![PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::UserManagement),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::Api),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::MyPlan),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::None]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::AppsManagement),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::CreateSubOrganizations),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::SubOrganizationGroups),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::Create, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::EditDelete]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::ManageSubOrganizations),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::Security),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::None]),
    ///             ..Default::default()
    ///         }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    ///             feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::Analytics),
    ///             permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::CreateAlerts, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::DownloadData, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::MyLooks, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::ExploreCreate]),
    ///             ..Default::default()
    ///         }]
    ///     }, None).await;
    /// }
    /// ```
    pub async fn change_admin_user_permissions(
        &self,
        email: &str,
        request: &PutCorporateUserEmailPermissionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("corporate/user/{}/permissions", email),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
