pub use crate::prelude::*;

/// Query parameters for getCorporateInvitedUsersList
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateInvitedUsersListQueryRequest {
    /// User type (active | pending). This is required if offset is provided for
    /// limited result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Page number for the result set. This is optional, default value will be
    /// the 1st page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Number of users to be displayed on each page. This is optional, the
    /// default limit is 20, but max allowed limit is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetCorporateInvitedUsersListQueryRequest {
    pub fn builder() -> GetCorporateInvitedUsersListQueryRequestBuilder {
        <GetCorporateInvitedUsersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateInvitedUsersListQueryRequestBuilder {
    r#type: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
}

impl GetCorporateInvitedUsersListQueryRequestBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateInvitedUsersListQueryRequest`].
    pub fn build(self) -> Result<GetCorporateInvitedUsersListQueryRequest, BuildError> {
        Ok(GetCorporateInvitedUsersListQueryRequest {
            r#type: self.r#type,
            offset: self.offset,
            limit: self.limit,
        })
    }
}
