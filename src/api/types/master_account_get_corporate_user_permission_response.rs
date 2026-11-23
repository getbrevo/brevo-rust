pub use crate::prelude::*;

/// Check admin user permissions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateUserPermissionResponse {
    /// Email address of the user.
    #[serde(default)]
    pub email: String,
    /// Granular feature permissions given to the user.
    #[serde(default)]
    pub feature_access: GetCorporateUserPermissionResponseFeatureAccess,
    #[serde(default)]
    pub groups: Vec<GetCorporateUserPermissionResponseGroupsItem>,
    /// Status of the invited user.
    #[serde(default)]
    pub status: String,
}

impl GetCorporateUserPermissionResponse {
    pub fn builder() -> GetCorporateUserPermissionResponseBuilder {
        <GetCorporateUserPermissionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateUserPermissionResponseBuilder {
    email: Option<String>,
    feature_access: Option<GetCorporateUserPermissionResponseFeatureAccess>,
    groups: Option<Vec<GetCorporateUserPermissionResponseGroupsItem>>,
    status: Option<String>,
}

impl GetCorporateUserPermissionResponseBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn feature_access(
        mut self,
        value: GetCorporateUserPermissionResponseFeatureAccess,
    ) -> Self {
        self.feature_access = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<GetCorporateUserPermissionResponseGroupsItem>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateUserPermissionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](GetCorporateUserPermissionResponseBuilder::email)
    /// - [`feature_access`](GetCorporateUserPermissionResponseBuilder::feature_access)
    /// - [`groups`](GetCorporateUserPermissionResponseBuilder::groups)
    /// - [`status`](GetCorporateUserPermissionResponseBuilder::status)
    pub fn build(self) -> Result<GetCorporateUserPermissionResponse, BuildError> {
        Ok(GetCorporateUserPermissionResponse {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            feature_access: self
                .feature_access
                .ok_or_else(|| BuildError::missing_field("feature_access"))?,
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
