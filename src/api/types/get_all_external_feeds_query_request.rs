pub use crate::prelude::*;

/// Query parameters for getAllExternalFeeds
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllExternalFeedsQueryRequest {
    /// Can be used to filter records by search keyword on feed name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Mandatory if `endDate` is used. Starting date (YYYY-MM-DD) from which
    /// you want to fetch the list. Can be maximum 30 days older than current
    /// date.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Mandatory if `startDate` is used. Ending date (YYYY-MM-DD) till which
    /// you want to fetch the list. Maximum time period that can be selected is
    /// one month.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    /// Sort the results in the ascending/descending order of record creation.
    /// Default order is **descending** if `sort` is not passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetAllExternalFeedsRequestSort>,
    /// Filter the records by `authType` of the feed.
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<GetAllExternalFeedsRequestAuthType>,
    /// Number of documents returned per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document on the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl GetAllExternalFeedsQueryRequest {
    pub fn builder() -> GetAllExternalFeedsQueryRequestBuilder {
        <GetAllExternalFeedsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllExternalFeedsQueryRequestBuilder {
    search: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    sort: Option<GetAllExternalFeedsRequestSort>,
    auth_type: Option<GetAllExternalFeedsRequestAuthType>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl GetAllExternalFeedsQueryRequestBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn sort(mut self, value: GetAllExternalFeedsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn auth_type(mut self, value: GetAllExternalFeedsRequestAuthType) -> Self {
        self.auth_type = Some(value);
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

    /// Consumes the builder and constructs a [`GetAllExternalFeedsQueryRequest`].
    pub fn build(self) -> Result<GetAllExternalFeedsQueryRequest, BuildError> {
        Ok(GetAllExternalFeedsQueryRequest {
            search: self.search,
            start_date: self.start_date,
            end_date: self.end_date,
            sort: self.sort,
            auth_type: self.auth_type,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
