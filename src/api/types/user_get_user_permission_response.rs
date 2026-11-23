pub use crate::prelude::*;

/// Check user permission
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUserPermissionResponse {
    /// Email address of the user.
    #[serde(default)]
    pub email: String,
    /// Granular feature permissions given to the user.
    #[serde(default)]
    pub privileges: Vec<GetUserPermissionResponsePrivilegesItem>,
    /// Status of the invited user.
    #[serde(default)]
    pub status: String,
}

impl GetUserPermissionResponse {
    pub fn builder() -> GetUserPermissionResponseBuilder {
        <GetUserPermissionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUserPermissionResponseBuilder {
    email: Option<String>,
    privileges: Option<Vec<GetUserPermissionResponsePrivilegesItem>>,
    status: Option<String>,
}

impl GetUserPermissionResponseBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn privileges(mut self, value: Vec<GetUserPermissionResponsePrivilegesItem>) -> Self {
        self.privileges = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetUserPermissionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](GetUserPermissionResponseBuilder::email)
    /// - [`privileges`](GetUserPermissionResponseBuilder::privileges)
    /// - [`status`](GetUserPermissionResponseBuilder::status)
    pub fn build(self) -> Result<GetUserPermissionResponse, BuildError> {
        Ok(GetUserPermissionResponse {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            privileges: self
                .privileges
                .ok_or_else(|| BuildError::missing_field("privileges"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
