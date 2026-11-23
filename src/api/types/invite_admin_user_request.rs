pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteAdminUserRequest {
    /// All access to the features
    #[serde(default)]
    pub all_features_access: bool,
    /// Email address for the organization
    #[serde(default)]
    pub email: String,
    /// Ids of Group
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub privileges: Vec<InviteAdminUserRequestPrivilegesItem>,
}

impl InviteAdminUserRequest {
    pub fn builder() -> InviteAdminUserRequestBuilder {
        <InviteAdminUserRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteAdminUserRequestBuilder {
    all_features_access: Option<bool>,
    email: Option<String>,
    group_ids: Option<Vec<String>>,
    privileges: Option<Vec<InviteAdminUserRequestPrivilegesItem>>,
}

impl InviteAdminUserRequestBuilder {
    pub fn all_features_access(mut self, value: bool) -> Self {
        self.all_features_access = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn group_ids(mut self, value: Vec<String>) -> Self {
        self.group_ids = Some(value);
        self
    }

    pub fn privileges(mut self, value: Vec<InviteAdminUserRequestPrivilegesItem>) -> Self {
        self.privileges = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InviteAdminUserRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`all_features_access`](InviteAdminUserRequestBuilder::all_features_access)
    /// - [`email`](InviteAdminUserRequestBuilder::email)
    /// - [`privileges`](InviteAdminUserRequestBuilder::privileges)
    pub fn build(self) -> Result<InviteAdminUserRequest, BuildError> {
        Ok(InviteAdminUserRequest {
            all_features_access: self
                .all_features_access
                .ok_or_else(|| BuildError::missing_field("all_features_access"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            group_ids: self.group_ids,
            privileges: self
                .privileges
                .ok_or_else(|| BuildError::missing_field("privileges"))?,
        })
    }
}
