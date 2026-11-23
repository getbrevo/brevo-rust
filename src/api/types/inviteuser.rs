pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Inviteuser {
    /// All access to the features
    #[serde(default)]
    pub all_features_access: bool,
    /// Email address for the organization
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub privileges: Vec<InviteuserPrivilegesItem>,
}

impl Inviteuser {
    pub fn builder() -> InviteuserBuilder {
        <InviteuserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteuserBuilder {
    all_features_access: Option<bool>,
    email: Option<String>,
    privileges: Option<Vec<InviteuserPrivilegesItem>>,
}

impl InviteuserBuilder {
    pub fn all_features_access(mut self, value: bool) -> Self {
        self.all_features_access = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn privileges(mut self, value: Vec<InviteuserPrivilegesItem>) -> Self {
        self.privileges = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Inviteuser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`all_features_access`](InviteuserBuilder::all_features_access)
    /// - [`email`](InviteuserBuilder::email)
    /// - [`privileges`](InviteuserBuilder::privileges)
    pub fn build(self) -> Result<Inviteuser, BuildError> {
        Ok(Inviteuser {
            all_features_access: self
                .all_features_access
                .ok_or_else(|| BuildError::missing_field("all_features_access"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            privileges: self
                .privileges
                .ok_or_else(|| BuildError::missing_field("privileges"))?,
        })
    }
}
