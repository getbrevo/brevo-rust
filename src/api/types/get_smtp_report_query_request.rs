pub use crate::prelude::*;

/// Query parameters for getSmtpReport
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmtpReportQueryRequest {
    /// Number of documents returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD)
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD)
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of days in the past including today (positive integer, maximum 30). _Not compatible with 'startDate' and 'endDate'_
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Tag of the emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetSmtpReportRequestSort>,
}

impl GetSmtpReportQueryRequest {
    pub fn builder() -> GetSmtpReportQueryRequestBuilder {
        <GetSmtpReportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmtpReportQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    start_date: Option<String>,
    end_date: Option<String>,
    days: Option<i64>,
    tag: Option<String>,
    sort: Option<GetSmtpReportRequestSort>,
}

impl GetSmtpReportQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn sort(mut self, value: GetSmtpReportRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmtpReportQueryRequest`].
    pub fn build(self) -> Result<GetSmtpReportQueryRequest, BuildError> {
        Ok(GetSmtpReportQueryRequest {
            limit: self.limit,
            offset: self.offset,
            start_date: self.start_date,
            end_date: self.end_date,
            days: self.days,
            tag: self.tag,
            sort: self.sort,
        })
    }
}
