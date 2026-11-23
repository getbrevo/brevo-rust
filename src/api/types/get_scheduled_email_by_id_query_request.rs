pub use crate::prelude::*;

/// Query parameters for getScheduledEmailById
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetScheduledEmailByIdQueryRequest {
    /// Mandatory if `endDate` is used. Starting date (YYYY-MM-DD) from which you want to fetch the list. Cannot be more than 30 days older than the current date.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Mandatory if `startDate` is used. Ending date (YYYY-MM-DD) till which you want to fetch the list. Maximum time period that can be selected is one month.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed. Not valid when identifier is `messageId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetScheduledEmailByIdRequestSort>,
    /// Filter the records by `status` of the scheduled email batch or message. Not valid when identifier is `messageId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetScheduledEmailByIdRequestStatus>,
    /// Number of documents returned per page. Not valid when identifier is `messageId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document on the page.  Not valid when identifier is `messageId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl GetScheduledEmailByIdQueryRequest {
    pub fn builder() -> GetScheduledEmailByIdQueryRequestBuilder {
        <GetScheduledEmailByIdQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetScheduledEmailByIdQueryRequestBuilder {
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    sort: Option<GetScheduledEmailByIdRequestSort>,
    status: Option<GetScheduledEmailByIdRequestStatus>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl GetScheduledEmailByIdQueryRequestBuilder {
    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn sort(mut self, value: GetScheduledEmailByIdRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn status(mut self, value: GetScheduledEmailByIdRequestStatus) -> Self {
        self.status = Some(value);
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

    /// Consumes the builder and constructs a [`GetScheduledEmailByIdQueryRequest`].
    pub fn build(self) -> Result<GetScheduledEmailByIdQueryRequest, BuildError> {
        Ok(GetScheduledEmailByIdQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            sort: self.sort,
            status: self.status,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
