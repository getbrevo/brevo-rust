pub use crate::prelude::*;

/// Query parameters for getFolders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetFoldersQueryRequest {
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetFoldersRequestSort>,
}

impl GetFoldersQueryRequest {
    pub fn builder() -> GetFoldersQueryRequestBuilder {
        <GetFoldersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetFoldersQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetFoldersRequestSort>,
}

impl GetFoldersQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetFoldersRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetFoldersQueryRequest`].
    pub fn build(self) -> Result<GetFoldersQueryRequest, BuildError> {
        Ok(GetFoldersQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
        })
    }
}
