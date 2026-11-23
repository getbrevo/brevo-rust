pub use crate::prelude::*;

/// Query parameters for getLPList
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLpListQueryRequest {
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort documents by field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<GetLpListRequestSortField>,
    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetLpListRequestSort>,
}

impl GetLpListQueryRequest {
    pub fn builder() -> GetLpListQueryRequestBuilder {
        <GetLpListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLpListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort_field: Option<GetLpListRequestSortField>,
    sort: Option<GetLpListRequestSort>,
}

impl GetLpListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort_field(mut self, value: GetLpListRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort(mut self, value: GetLpListRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLpListQueryRequest`].
    pub fn build(self) -> Result<GetLpListQueryRequest, BuildError> {
        Ok(GetLpListQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort_field: self.sort_field,
            sort: self.sort,
        })
    }
}
