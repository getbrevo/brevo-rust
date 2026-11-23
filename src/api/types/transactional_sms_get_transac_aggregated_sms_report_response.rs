pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacAggregatedSmsReportResponse {
    /// Number of accepted SMS for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted: Option<i64>,
    /// Number of blocked contact for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<i64>,
    /// Number of delivered SMS for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// Number of hardbounces for the timeframe
    #[serde(rename = "hardBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_bounces: Option<i64>,
    /// Time frame of the report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// Number of rejected SMS for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected: Option<i64>,
    /// Number of answered SMS for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied: Option<i64>,
    /// Number of requests for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    /// Number of skipped SMS for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    /// Number of softbounces for the timeframe
    #[serde(rename = "softBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_bounces: Option<i64>,
    /// Number of unsubscription for the timeframe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
}

impl GetTransacAggregatedSmsReportResponse {
    pub fn builder() -> GetTransacAggregatedSmsReportResponseBuilder {
        <GetTransacAggregatedSmsReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacAggregatedSmsReportResponseBuilder {
    accepted: Option<i64>,
    blocked: Option<i64>,
    delivered: Option<i64>,
    hard_bounces: Option<i64>,
    range: Option<String>,
    rejected: Option<i64>,
    replied: Option<i64>,
    requests: Option<i64>,
    skipped: Option<i64>,
    soft_bounces: Option<i64>,
    unsubscribed: Option<i64>,
}

impl GetTransacAggregatedSmsReportResponseBuilder {
    pub fn accepted(mut self, value: i64) -> Self {
        self.accepted = Some(value);
        self
    }

    pub fn blocked(mut self, value: i64) -> Self {
        self.blocked = Some(value);
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

    pub fn range(mut self, value: impl Into<String>) -> Self {
        self.range = Some(value.into());
        self
    }

    pub fn rejected(mut self, value: i64) -> Self {
        self.rejected = Some(value);
        self
    }

    pub fn replied(mut self, value: i64) -> Self {
        self.replied = Some(value);
        self
    }

    pub fn requests(mut self, value: i64) -> Self {
        self.requests = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn soft_bounces(mut self, value: i64) -> Self {
        self.soft_bounces = Some(value);
        self
    }

    pub fn unsubscribed(mut self, value: i64) -> Self {
        self.unsubscribed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTransacAggregatedSmsReportResponse`].
    pub fn build(self) -> Result<GetTransacAggregatedSmsReportResponse, BuildError> {
        Ok(GetTransacAggregatedSmsReportResponse {
            accepted: self.accepted,
            blocked: self.blocked,
            delivered: self.delivered,
            hard_bounces: self.hard_bounces,
            range: self.range,
            rejected: self.rejected,
            replied: self.replied,
            requests: self.requests,
            skipped: self.skipped,
            soft_bounces: self.soft_bounces,
            unsubscribed: self.unsubscribed,
        })
    }
}
