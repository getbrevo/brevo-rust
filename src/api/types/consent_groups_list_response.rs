pub use crate::prelude::*;

/// Paginated list of consent groups.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConsentGroupsListResponse {
    /// List of consent groups matching the filter
    #[serde(rename = "consentGroups")]
    #[serde(default)]
    pub consent_groups: Vec<ConsentGroup>,
    /// Total number of consent groups matching the filter
    #[serde(default)]
    pub total: i64,
    /// Maximum number of results returned
    #[serde(default)]
    pub limit: i64,
    /// Number of results skipped
    #[serde(default)]
    pub offset: i64,
}

impl ConsentGroupsListResponse {
    pub fn builder() -> ConsentGroupsListResponseBuilder {
        <ConsentGroupsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsentGroupsListResponseBuilder {
    consent_groups: Option<Vec<ConsentGroup>>,
    total: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl ConsentGroupsListResponseBuilder {
    pub fn consent_groups(mut self, value: Vec<ConsentGroup>) -> Self {
        self.consent_groups = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsentGroupsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`consent_groups`](ConsentGroupsListResponseBuilder::consent_groups)
    /// - [`total`](ConsentGroupsListResponseBuilder::total)
    /// - [`limit`](ConsentGroupsListResponseBuilder::limit)
    /// - [`offset`](ConsentGroupsListResponseBuilder::offset)
    pub fn build(self) -> Result<ConsentGroupsListResponse, BuildError> {
        Ok(ConsentGroupsListResponse {
            consent_groups: self
                .consent_groups
                .ok_or_else(|| BuildError::missing_field("consent_groups"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
            offset: self
                .offset
                .ok_or_else(|| BuildError::missing_field("offset"))?,
        })
    }
}
