pub use crate::prelude::*;

/// Query parameters for getCouponCollections
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCouponCollectionsQueryRequest {
    /// Number of documents returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results by creation time in ascending/descending order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetCouponCollectionsRequestSort>,
    /// The field used to sort coupon collections
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GetCouponCollectionsRequestSortBy>,
}

impl GetCouponCollectionsQueryRequest {
    pub fn builder() -> GetCouponCollectionsQueryRequestBuilder {
        <GetCouponCollectionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCouponCollectionsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetCouponCollectionsRequestSort>,
    sort_by: Option<GetCouponCollectionsRequestSortBy>,
}

impl GetCouponCollectionsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetCouponCollectionsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_by(mut self, value: GetCouponCollectionsRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCouponCollectionsQueryRequest`].
    pub fn build(self) -> Result<GetCouponCollectionsQueryRequest, BuildError> {
        Ok(GetCouponCollectionsQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            sort_by: self.sort_by,
        })
    }
}
