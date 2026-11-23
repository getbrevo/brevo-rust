pub use crate::prelude::*;

/// Query parameters for getBalanceDefinitionList
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBalanceDefinitionListQueryRequest {
    /// Limit the number of records returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset to paginate records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Field to sort by
    #[serde(rename = "sortField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<GetBalanceDefinitionListRequestSortField>,
    /// Sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetBalanceDefinitionListRequestSort>,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetBalanceDefinitionListRequestVersion>,
}

impl GetBalanceDefinitionListQueryRequest {
    pub fn builder() -> GetBalanceDefinitionListQueryRequestBuilder {
        <GetBalanceDefinitionListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBalanceDefinitionListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort_field: Option<GetBalanceDefinitionListRequestSortField>,
    sort: Option<GetBalanceDefinitionListRequestSort>,
    version: Option<GetBalanceDefinitionListRequestVersion>,
}

impl GetBalanceDefinitionListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort_field(mut self, value: GetBalanceDefinitionListRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort(mut self, value: GetBalanceDefinitionListRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn version(mut self, value: GetBalanceDefinitionListRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBalanceDefinitionListQueryRequest`].
    pub fn build(self) -> Result<GetBalanceDefinitionListQueryRequest, BuildError> {
        Ok(GetBalanceDefinitionListQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort_field: self.sort_field,
            sort: self.sort,
            version: self.version,
        })
    }
}
