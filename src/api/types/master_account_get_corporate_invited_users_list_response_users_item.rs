pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateInvitedUsersListResponseUsersItem {
    /// Email address of the user.
    #[serde(default)]
    pub email: String,
    /// Feature accessiblity given to the user. (Required only
    /// if status is active)
    #[serde(default)]
    pub feature_access: GetCorporateInvitedUsersListResponseUsersItemFeatureAccess,
    /// Admin user groups list
    #[serde(default)]
    pub groups: GetCorporateInvitedUsersListResponseUsersItemGroups,
    /// Flag for indicating is user owner of the organization.
    #[serde(default)]
    pub is_owner: String,
    /// Status of the invited user.
    #[serde(default)]
    pub status: String,
}

impl GetCorporateInvitedUsersListResponseUsersItem {
    pub fn builder() -> GetCorporateInvitedUsersListResponseUsersItemBuilder {
        <GetCorporateInvitedUsersListResponseUsersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateInvitedUsersListResponseUsersItemBuilder {
    email: Option<String>,
    feature_access: Option<GetCorporateInvitedUsersListResponseUsersItemFeatureAccess>,
    groups: Option<GetCorporateInvitedUsersListResponseUsersItemGroups>,
    is_owner: Option<String>,
    status: Option<String>,
}

impl GetCorporateInvitedUsersListResponseUsersItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn feature_access(
        mut self,
        value: GetCorporateInvitedUsersListResponseUsersItemFeatureAccess,
    ) -> Self {
        self.feature_access = Some(value);
        self
    }

    pub fn groups(mut self, value: GetCorporateInvitedUsersListResponseUsersItemGroups) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn is_owner(mut self, value: impl Into<String>) -> Self {
        self.is_owner = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateInvitedUsersListResponseUsersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](GetCorporateInvitedUsersListResponseUsersItemBuilder::email)
    /// - [`feature_access`](GetCorporateInvitedUsersListResponseUsersItemBuilder::feature_access)
    /// - [`groups`](GetCorporateInvitedUsersListResponseUsersItemBuilder::groups)
    /// - [`is_owner`](GetCorporateInvitedUsersListResponseUsersItemBuilder::is_owner)
    /// - [`status`](GetCorporateInvitedUsersListResponseUsersItemBuilder::status)
    pub fn build(self) -> Result<GetCorporateInvitedUsersListResponseUsersItem, BuildError> {
        Ok(GetCorporateInvitedUsersListResponseUsersItem {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            feature_access: self
                .feature_access
                .ok_or_else(|| BuildError::missing_field("feature_access"))?,
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            is_owner: self
                .is_owner
                .ok_or_else(|| BuildError::missing_field("is_owner"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
