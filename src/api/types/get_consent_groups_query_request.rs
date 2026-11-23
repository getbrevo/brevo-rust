pub use crate::prelude::*;

/// Query parameters for getConsentGroups
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetConsentGroupsQueryRequest {
    /// Maximum number of results to return (default 10, max 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of results to skip (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter by consent group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Filter by name (case-insensitive partial match)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by signup mode
    #[serde(rename = "signupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_mode: Option<GetConsentGroupsRequestSignupMode>,
}

impl GetConsentGroupsQueryRequest {
    pub fn builder() -> GetConsentGroupsQueryRequestBuilder {
        <GetConsentGroupsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConsentGroupsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    id: Option<i64>,
    name: Option<String>,
    signup_mode: Option<GetConsentGroupsRequestSignupMode>,
}

impl GetConsentGroupsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn signup_mode(mut self, value: GetConsentGroupsRequestSignupMode) -> Self {
        self.signup_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetConsentGroupsQueryRequest`].
    pub fn build(self) -> Result<GetConsentGroupsQueryRequest, BuildError> {
        Ok(GetConsentGroupsQueryRequest {
            limit: self.limit,
            offset: self.offset,
            id: self.id,
            name: self.name,
            signup_mode: self.signup_mode,
        })
    }
}
