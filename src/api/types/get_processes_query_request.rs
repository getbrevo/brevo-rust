pub use crate::prelude::*;

/// Query parameters for getProcesses
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProcessesQueryRequest {
    /// Number limitation for the result returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Beginning point in the list to retrieve from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation.
    /// Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetProcessesRequestSort>,
}

impl GetProcessesQueryRequest {
    pub fn builder() -> GetProcessesQueryRequestBuilder {
        <GetProcessesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProcessesQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetProcessesRequestSort>,
}

impl GetProcessesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetProcessesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProcessesQueryRequest`].
    pub fn build(self) -> Result<GetProcessesQueryRequest, BuildError> {
        Ok(GetProcessesQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
        })
    }
}
