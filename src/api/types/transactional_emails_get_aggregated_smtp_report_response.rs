pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAggregatedSmtpReportResponse {
    /// Number of blocked contact emails for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<i64>,
    /// Number of clicks for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// Number of delivered emails for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// Number of hardbounces for the timeframe
    #[serde(rename = "hardBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_bounces: Option<i64>,
    /// Number of invalid emails for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<i64>,
    /// Number of openings for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    /// Time frame of the report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// Number of requests for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    /// Number of softbounces for the timeframe
    #[serde(rename = "softBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_bounces: Option<i64>,
    /// Number of complaint (spam report) for the timeframe
    #[serde(rename = "spamReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_reports: Option<i64>,
    /// Number of unique clicks for the timeframe
    #[serde(rename = "uniqueClicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i64>,
    /// Number of unique openings for the timeframe
    #[serde(rename = "uniqueOpens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
    /// Number of unsubscribed emails for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
}

impl GetAggregatedSmtpReportResponse {
    pub fn builder() -> GetAggregatedSmtpReportResponseBuilder {
        <GetAggregatedSmtpReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAggregatedSmtpReportResponseBuilder {
    blocked: Option<i64>,
    clicks: Option<i64>,
    delivered: Option<i64>,
    hard_bounces: Option<i64>,
    invalid: Option<i64>,
    opens: Option<i64>,
    range: Option<String>,
    requests: Option<i64>,
    soft_bounces: Option<i64>,
    spam_reports: Option<i64>,
    unique_clicks: Option<i64>,
    unique_opens: Option<i64>,
    unsubscribed: Option<i64>,
}

impl GetAggregatedSmtpReportResponseBuilder {
    pub fn blocked(mut self, value: i64) -> Self {
        self.blocked = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn delivered(mut self, value: i64) -> Self {
        self.delivered = Some(value);
        self
    }

    pub fn hard_bounces(mut self, value: i64) -> Self {
        self.hard_bounces = Some(value);
        self
    }

    pub fn invalid(mut self, value: i64) -> Self {
        self.invalid = Some(value);
        self
    }

    pub fn opens(mut self, value: i64) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn range(mut self, value: impl Into<String>) -> Self {
        self.range = Some(value.into());
        self
    }

    pub fn requests(mut self, value: i64) -> Self {
        self.requests = Some(value);
        self
    }

    pub fn soft_bounces(mut self, value: i64) -> Self {
        self.soft_bounces = Some(value);
        self
    }

    pub fn spam_reports(mut self, value: i64) -> Self {
        self.spam_reports = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: i64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    pub fn unsubscribed(mut self, value: i64) -> Self {
        self.unsubscribed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAggregatedSmtpReportResponse`].
    pub fn build(self) -> Result<GetAggregatedSmtpReportResponse, BuildError> {
        Ok(GetAggregatedSmtpReportResponse {
            blocked: self.blocked,
            clicks: self.clicks,
            delivered: self.delivered,
            hard_bounces: self.hard_bounces,
            invalid: self.invalid,
            opens: self.opens,
            range: self.range,
            requests: self.requests,
            soft_bounces: self.soft_bounces,
            spam_reports: self.spam_reports,
            unique_clicks: self.unique_clicks,
            unique_opens: self.unique_opens,
            unsubscribed: self.unsubscribed,
        })
    }
}
