pub use crate::prelude::*;

/// Admin user groups list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateInvitedUsersListResponseUsersItemGroups {
    /// group id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetCorporateInvitedUsersListResponseUsersItemGroups {
    pub fn builder() -> GetCorporateInvitedUsersListResponseUsersItemGroupsBuilder {
        <GetCorporateInvitedUsersListResponseUsersItemGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateInvitedUsersListResponseUsersItemGroupsBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl GetCorporateInvitedUsersListResponseUsersItemGroupsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateInvitedUsersListResponseUsersItemGroups`].
    pub fn build(self) -> Result<GetCorporateInvitedUsersListResponseUsersItemGroups, BuildError> {
        Ok(GetCorporateInvitedUsersListResponseUsersItemGroups {
            id: self.id,
            name: self.name,
        })
    }
}
