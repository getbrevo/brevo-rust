pub use crate::prelude::*;

/// Query parameters for getAggregatedSmtpReport
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAggregatedSmtpReportQueryRequest {
    /// **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of days in the past including today (positive integer, maximum 90). _Not compatible with 'startDate' and 'endDate'_. Defaults to 90 if neither dates nor days are provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Tag of the emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl GetAggregatedSmtpReportQueryRequest {
    pub fn builder() -> GetAggregatedSmtpReportQueryRequestBuilder {
        <GetAggregatedSmtpReportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAggregatedSmtpReportQueryRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    days: Option<i64>,
    tag: Option<String>,
}

impl GetAggregatedSmtpReportQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`GetAggregatedSmtpReportQueryRequest`].
    pub fn build(self) -> Result<GetAggregatedSmtpReportQueryRequest, BuildError> {
        Ok(GetAggregatedSmtpReportQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            days: self.days,
            tag: self.tag,
        })
    }
}
