pub use crate::prelude::*;

/// Query parameters for getSMSTemplates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsTemplatesQueryRequest {
    /// Number of documents returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetSmsTemplatesRequestSort>,
}

impl GetSmsTemplatesQueryRequest {
    pub fn builder() -> GetSmsTemplatesQueryRequestBuilder {
        <GetSmsTemplatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsTemplatesQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetSmsTemplatesRequestSort>,
}

impl GetSmsTemplatesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetSmsTemplatesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsTemplatesQueryRequest`].
    pub fn build(self) -> Result<GetSmsTemplatesQueryRequest, BuildError> {
        Ok(GetSmsTemplatesQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
        })
    }
}
