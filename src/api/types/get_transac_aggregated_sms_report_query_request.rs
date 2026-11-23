pub use crate::prelude::*;

/// Query parameters for getTransacAggregatedSmsReport
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacAggregatedSmsReportQueryRequest {
    /// **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of days in the past including today (positive integer). **Not compatible with startDate and endDate**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Filter on a tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl GetTransacAggregatedSmsReportQueryRequest {
    pub fn builder() -> GetTransacAggregatedSmsReportQueryRequestBuilder {
        <GetTransacAggregatedSmsReportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacAggregatedSmsReportQueryRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    days: Option<i64>,
    tag: Option<String>,
}

impl GetTransacAggregatedSmsReportQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`GetTransacAggregatedSmsReportQueryRequest`].
    pub fn build(self) -> Result<GetTransacAggregatedSmsReportQueryRequest, BuildError> {
        Ok(GetTransacAggregatedSmsReportQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            days: self.days,
            tag: self.tag,
        })
    }
}
