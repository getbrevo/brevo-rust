pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateInvitedUsersListResponse {
    /// Get invited users list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GetCorporateInvitedUsersListResponseUsersItem>>,
}

impl GetCorporateInvitedUsersListResponse {
    pub fn builder() -> GetCorporateInvitedUsersListResponseBuilder {
        <GetCorporateInvitedUsersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateInvitedUsersListResponseBuilder {
    users: Option<Vec<GetCorporateInvitedUsersListResponseUsersItem>>,
}

impl GetCorporateInvitedUsersListResponseBuilder {
    pub fn users(mut self, value: Vec<GetCorporateInvitedUsersListResponseUsersItem>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateInvitedUsersListResponse`].
    pub fn build(self) -> Result<GetCorporateInvitedUsersListResponse, BuildError> {
        Ok(GetCorporateInvitedUsersListResponse { users: self.users })
    }
}
