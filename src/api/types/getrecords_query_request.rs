pub use crate::prelude::*;

/// Query parameters for getrecords
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetrecordsQueryRequest {
    /// Number of records returned per page
    #[serde(default)]
    pub limit: i64,
    /// Page number for pagination. It's used to fetch the object records on a provided page number. Must be a valid positive integer.
    #[serde(default)]
    pub page_num: i64,
    /// Sort order, must be 'asc' or 'desc'. Default to 'desc' if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetrecordsRequestSort>,
    /// Whether to include associations, must be 'true' or 'false'. Default to 'false' if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<GetrecordsRequestAssociation>,
}

impl GetrecordsQueryRequest {
    pub fn builder() -> GetrecordsQueryRequestBuilder {
        <GetrecordsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsQueryRequestBuilder {
    limit: Option<i64>,
    page_num: Option<i64>,
    sort: Option<GetrecordsRequestSort>,
    association: Option<GetrecordsRequestAssociation>,
}

impl GetrecordsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page_num(mut self, value: i64) -> Self {
        self.page_num = Some(value);
        self
    }

    pub fn sort(mut self, value: GetrecordsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn association(mut self, value: GetrecordsRequestAssociation) -> Self {
        self.association = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limit`](GetrecordsQueryRequestBuilder::limit)
    /// - [`page_num`](GetrecordsQueryRequestBuilder::page_num)
    pub fn build(self) -> Result<GetrecordsQueryRequest, BuildError> {
        Ok(GetrecordsQueryRequest {
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
            page_num: self
                .page_num
                .ok_or_else(|| BuildError::missing_field("page_num"))?,
            sort: self.sort,
            association: self.association,
        })
    }
}
