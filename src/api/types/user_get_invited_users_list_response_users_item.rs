pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInvitedUsersListResponseUsersItem {
    /// Email address of the user.
    #[serde(default)]
    pub email: String,
    /// Feature accessiblity given to the user.
    #[serde(default)]
    pub feature_access: GetInvitedUsersListResponseUsersItemFeatureAccess,
    /// Flag for indicating is user owner of the organization.
    #[serde(default)]
    pub is_owner: String,
    /// Status of the invited user.
    #[serde(default)]
    pub status: String,
}

impl GetInvitedUsersListResponseUsersItem {
    pub fn builder() -> GetInvitedUsersListResponseUsersItemBuilder {
        <GetInvitedUsersListResponseUsersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInvitedUsersListResponseUsersItemBuilder {
    email: Option<String>,
    feature_access: Option<GetInvitedUsersListResponseUsersItemFeatureAccess>,
    is_owner: Option<String>,
    status: Option<String>,
}

impl GetInvitedUsersListResponseUsersItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn feature_access(
        mut self,
        value: GetInvitedUsersListResponseUsersItemFeatureAccess,
    ) -> Self {
        self.feature_access = Some(value);
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

    /// Consumes the builder and constructs a [`GetInvitedUsersListResponseUsersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](GetInvitedUsersListResponseUsersItemBuilder::email)
    /// - [`feature_access`](GetInvitedUsersListResponseUsersItemBuilder::feature_access)
    /// - [`is_owner`](GetInvitedUsersListResponseUsersItemBuilder::is_owner)
    /// - [`status`](GetInvitedUsersListResponseUsersItemBuilder::status)
    pub fn build(self) -> Result<GetInvitedUsersListResponseUsersItem, BuildError> {
        Ok(GetInvitedUsersListResponseUsersItem {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            feature_access: self
                .feature_access
                .ok_or_else(|| BuildError::missing_field("feature_access"))?,
            is_owner: self
                .is_owner
                .ok_or_else(|| BuildError::missing_field("is_owner"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
