pub use crate::prelude::*;

/// Query parameters for getOrders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrdersQueryRequest {
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetOrdersRequestSort>,
    /// When `true`, orders with the same `createdAt` are then ordered by their `updatedAt`, in the same direction as `sort`. Use it to get a stable order when paging through the results with `limit` and `offset`. Default is `false`.
    #[serde(rename = "secondarySortByModifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_sort_by_modified_at: Option<bool>,
    /// Filter (urlencoded) the orders modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since: Option<String>,
    /// Filter (urlencoded) the orders created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "createdSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_since: Option<String>,
}

impl GetOrdersQueryRequest {
    pub fn builder() -> GetOrdersQueryRequestBuilder {
        <GetOrdersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrdersQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetOrdersRequestSort>,
    secondary_sort_by_modified_at: Option<bool>,
    modified_since: Option<String>,
    created_since: Option<String>,
}

impl GetOrdersQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetOrdersRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn secondary_sort_by_modified_at(mut self, value: bool) -> Self {
        self.secondary_sort_by_modified_at = Some(value);
        self
    }

    pub fn modified_since(mut self, value: impl Into<String>) -> Self {
        self.modified_since = Some(value.into());
        self
    }

    pub fn created_since(mut self, value: impl Into<String>) -> Self {
        self.created_since = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrdersQueryRequest`].
    pub fn build(self) -> Result<GetOrdersQueryRequest, BuildError> {
        Ok(GetOrdersQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            secondary_sort_by_modified_at: self.secondary_sort_by_modified_at,
            modified_since: self.modified_since,
            created_since: self.created_since,
        })
    }
}
