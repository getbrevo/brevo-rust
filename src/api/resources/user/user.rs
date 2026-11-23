use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct UserClient {
    pub http_client: HttpClient,
}

impl UserClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves the list of all users associated with your organization, including both active and pending invited users. Each user entry includes their email address, owner status, current invitation status, and feature access levels for marketing, CRM, and conversations.
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
    ///     client.user.get_invited_users_list(None).await;
    /// }
    /// ```
    pub async fn get_invited_users_list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetInvitedUsersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "organization/invited/users",
                None,
                None,
                options,
            )
            .await
    }

    /// Revokes all permissions for an invited user in the organization, effectively removing their access to the platform. If the user''s plan change generated credit notes, they are returned in the response for billing reconciliation.
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
    ///         .user
    ///         .put_revoke_user_permission(&"email".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn put_revoke_user_permission(
        &self,
        email: &str,
        options: Option<RequestOptions>,
    ) -> Result<PutRevokeUserPermissionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("organization/user/invitation/revoke/{}", email),
                None,
                None,
                options,
            )
            .await
    }

    /// `Feature` - A Feature represents a specific functionality like Email
    /// campaign, Deals, Calls, Automations, etc. on Brevo. While inviting a user,
    /// determine which feature you want to manage access to. You must specify the
    /// feature accurately to avoid errors. `Permission` - A Permission defines the
    /// level of access or control a user has over a specific feature. While
    /// inviting user, decide on the permission level required for the selected
    /// feature. Make sure the chosen permission is related to the selected feature.
    /// Features and their respective permissions are as below: - `email_campaigns`:
    /// - "create_edit_delete"
    /// - "send_schedule_suspend"
    /// - `sms_campaigns`:
    /// - "create_edit_delete"
    /// - "send_schedule_suspend"
    /// - `contacts`:
    /// - "view"
    /// - "create_edit_delete"
    /// - "import"
    /// - "export"
    /// - "list_and_attributes"
    /// - "forms"
    /// - `templates`:
    /// - "create_edit_delete"
    /// - "activate_deactivate"
    /// - `workflows`:
    /// - "create_edit_delete"
    /// - "activate_deactivate_pause"
    /// - "settings"
    /// - `landing_pages`:
    /// - "all"
    /// - `transactional_emails`:
    /// - "settings"
    /// - "logs"
    /// - `smtp_api`:
    /// - "smtp"
    /// - "api_keys"
    /// - "authorized_ips"
    /// - `user_management`:
    /// - "all"
    /// - `sales_platform`:
    /// - "create_edit_deals"
    /// - "delete_deals"
    /// - "manage_others_deals_tasks"
    /// - "reports"
    /// - "settings"
    /// - `phone`:
    /// - "all"
    /// - `conversations`:
    /// - "access"
    /// - "assign"
    /// - "configure"
    /// - `senders_domains_dedicated_ips`:
    /// - "senders_management"
    /// - "domains_management"
    /// - "dedicated_ips_management"
    /// - `push_notifications`:
    /// - "view"
    /// - "create_edit_delete"
    /// - "send"
    /// - "settings"
    /// - `companies`:
    /// - "manage_owned_companies"
    /// - "manage_other_companies"
    /// - "settings"
    /// **Note**: - If `all_features_access: false` then only privileges are
    /// required otherwise if `true` then it's assumed that all permissions will be
    /// there for the invited user. - The availability of feature and its permission
    /// depends on your current plan. Please select the features and permissions
    /// accordingly.
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
    ///         .user
    ///         .inviteuser(
    ///             &Inviteuser {
    ///                 all_features_access: true,
    ///                 email: "inviteuser@example.com".to_string(),
    ///                 privileges: vec![InviteuserPrivilegesItem {
    ///                     ..Default::default()
    ///                 }],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn inviteuser(
        &self,
        request: &Inviteuser,
        options: Option<RequestOptions>,
    ) -> Result<InviteuserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "organization/user/invitation/send",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Resends or cancels a pending invitation for a user in the organization, depending on the action path parameter. Use `resend` to send a new invitation email to the user, or `cancel` to revoke the pending invitation entirely and remove the user''s pending access.
    ///
    /// # Arguments
    ///
    /// * `action` - action
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
    ///         .user
    ///         .putresendcancelinvitation(
    ///             &PutresendcancelinvitationRequestAction::Resend,
    ///             &"email".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn putresendcancelinvitation(
        &self,
        action: &PutresendcancelinvitationRequestAction,
        email: &str,
        options: Option<RequestOptions>,
    ) -> Result<PutresendcancelinvitationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("organization/user/invitation/{}/{}", action, email),
                None,
                None,
                options,
            )
            .await
    }

    /// `Feature` - A Feature represents a specific functionality like Email
    /// campaign, Deals, Calls, Automations, etc. on Brevo. While inviting a user,
    /// determine which feature you want to manage access to. You must specify the
    /// feature accurately to avoid errors. `Permission` - A Permission defines the
    /// level of access or control a user has over a specific feature. While
    /// inviting user, decide on the permission level required for the selected
    /// feature. Make sure the chosen permission is related to the selected feature.
    /// Features and their respective permissions are as below: - `email_campaigns`:
    /// - "create_edit_delete"
    /// - "send_schedule_suspend"
    /// - `sms_campaigns`:
    /// - "create_edit_delete"
    /// - "send_schedule_suspend"
    /// - `contacts`:
    /// - "view"
    /// - "create_edit_delete"
    /// - "import"
    /// - "export"
    /// - "list_and_attributes"
    /// - "forms"
    /// - `templates`:
    /// - "create_edit_delete"
    /// - "activate_deactivate"
    /// - `workflows`:
    /// - "create_edit_delete"
    /// - "activate_deactivate_pause"
    /// - "settings"
    /// - `landing_pages`:
    /// - "all"
    /// - `transactional_emails`:
    /// - "settings"
    /// - "logs"
    /// - `smtp_api`:
    /// - "smtp"
    /// - "api_keys"
    /// - "authorized_ips"
    /// - `user_management`:
    /// - "all"
    /// - `sales_platform`:
    /// - "create_edit_deals"
    /// - "delete_deals"
    /// - "manage_others_deals_tasks"
    /// - "reports"
    /// - "settings"
    /// - `phone`:
    /// - "all"
    /// - `conversations`:
    /// - "access"
    /// - "assign"
    /// - "configure"
    /// - `senders_domains_dedicated_ips`:
    /// - "senders_management"
    /// - "domains_management"
    /// - "dedicated_ips_management"
    /// - `push_notifications`:
    /// - "view"
    /// - "create_edit_delete"
    /// - "send"
    /// - "settings"
    /// - `companies`:
    /// - "manage_owned_companies"
    /// - "manage_other_companies"
    /// - "settings"
    /// **Note**: - The privileges array remains the same as in the send invitation;
    /// the user simply needs to provide the permissions that need to be updated. -
    /// The availability of feature and its permission depends on your current plan.
    /// Please select the features and permissions accordingly.
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
    ///         .user
    ///         .edit_user_permission(
    ///             &Inviteuser {
    ///                 all_features_access: true,
    ///                 email: "inviteuser@example.com".to_string(),
    ///                 privileges: vec![InviteuserPrivilegesItem {
    ///                     ..Default::default()
    ///                 }],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn edit_user_permission(
        &self,
        request: &Inviteuser,
        options: Option<RequestOptions>,
    ) -> Result<EditUserPermissionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "organization/user/update/permissions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the granular feature-level permissions assigned to a specific user in the organization, identified by their email address. The response includes the user''s current status (active or pending) and a detailed list of privileges specifying which features and permission levels are granted.
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
    ///         .user
    ///         .get_user_permission(&"email".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_user_permission(
        &self,
        email: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetUserPermissionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("organization/user/{}/permissions", email),
                None,
                None,
                options,
            )
            .await
    }
}
