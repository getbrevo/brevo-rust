pub use crate::prelude::*;

/// Feature accessiblity given to the user.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInvitedUsersListResponseUsersItemFeatureAccess {
    /// Conversations features accessiblity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<String>,
    /// CRM features accessiblity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crm: Option<String>,
    /// Marketing features accessiblity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing: Option<String>,
}

impl GetInvitedUsersListResponseUsersItemFeatureAccess {
    pub fn builder() -> GetInvitedUsersListResponseUsersItemFeatureAccessBuilder {
        <GetInvitedUsersListResponseUsersItemFeatureAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInvitedUsersListResponseUsersItemFeatureAccessBuilder {
    conversations: Option<String>,
    crm: Option<String>,
    marketing: Option<String>,
}

impl GetInvitedUsersListResponseUsersItemFeatureAccessBuilder {
    pub fn conversations(mut self, value: impl Into<String>) -> Self {
        self.conversations = Some(value.into());
        self
    }

    pub fn crm(mut self, value: impl Into<String>) -> Self {
        self.crm = Some(value.into());
        self
    }

    pub fn marketing(mut self, value: impl Into<String>) -> Self {
        self.marketing = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetInvitedUsersListResponseUsersItemFeatureAccess`].
    pub fn build(self) -> Result<GetInvitedUsersListResponseUsersItemFeatureAccess, BuildError> {
        Ok(GetInvitedUsersListResponseUsersItemFeatureAccess {
            conversations: self.conversations,
            crm: self.crm,
            marketing: self.marketing,
        })
    }
}
