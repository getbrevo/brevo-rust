pub use crate::prelude::*;

/// Query parameters for getTransacSmsReport
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacSmsReportQueryRequest {
    /// **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of days in the past including today (positive integer). **Not compatible with 'startDate' and 'endDate'**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Filter on a tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetTransacSmsReportRequestSort>,
}

impl GetTransacSmsReportQueryRequest {
    pub fn builder() -> GetTransacSmsReportQueryRequestBuilder {
        <GetTransacSmsReportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacSmsReportQueryRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    days: Option<i64>,
    tag: Option<String>,
    sort: Option<GetTransacSmsReportRequestSort>,
}

impl GetTransacSmsReportQueryRequestBuilder {
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

    pub fn sort(mut self, value: GetTransacSmsReportRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTransacSmsReportQueryRequest`].
    pub fn build(self) -> Result<GetTransacSmsReportQueryRequest, BuildError> {
        Ok(GetTransacSmsReportQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            days: self.days,
            tag: self.tag,
            sort: self.sort,
        })
    }
}
