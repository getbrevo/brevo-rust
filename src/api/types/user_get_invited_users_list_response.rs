pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInvitedUsersListResponse {
    /// Get invited users list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GetInvitedUsersListResponseUsersItem>>,
}

impl GetInvitedUsersListResponse {
    pub fn builder() -> GetInvitedUsersListResponseBuilder {
        <GetInvitedUsersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInvitedUsersListResponseBuilder {
    users: Option<Vec<GetInvitedUsersListResponseUsersItem>>,
}

impl GetInvitedUsersListResponseBuilder {
    pub fn users(mut self, value: Vec<GetInvitedUsersListResponseUsersItem>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetInvitedUsersListResponse`].
    pub fn build(self) -> Result<GetInvitedUsersListResponse, BuildError> {
        Ok(GetInvitedUsersListResponse { users: self.users })
    }
}
