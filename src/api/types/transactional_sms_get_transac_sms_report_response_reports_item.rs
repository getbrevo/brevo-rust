pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacSmsReportResponseReportsItem {
    /// Number of accepted SMS for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted: Option<i64>,
    /// Number of blocked contact for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<i64>,
    /// Date for which statistics are retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    /// Number of delivered SMS for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// Number of hardbounces for the date
    #[serde(rename = "hardBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_bounces: Option<i64>,
    /// Number of rejected SMS for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected: Option<i64>,
    /// Number of answered SMS for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied: Option<i64>,
    /// Number of requests for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    /// Number of skipped SMS for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    /// Number of softbounces for the date
    #[serde(rename = "softBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_bounces: Option<i64>,
    /// Number of unsubscription for the date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
}

impl GetTransacSmsReportResponseReportsItem {
    pub fn builder() -> GetTransacSmsReportResponseReportsItemBuilder {
        <GetTransacSmsReportResponseReportsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacSmsReportResponseReportsItemBuilder {
    accepted: Option<i64>,
    blocked: Option<i64>,
    date: Option<NaiveDate>,
    delivered: Option<i64>,
    hard_bounces: Option<i64>,
    rejected: Option<i64>,
    replied: Option<i64>,
    requests: Option<i64>,
    skipped: Option<i64>,
    soft_bounces: Option<i64>,
    unsubscribed: Option<i64>,
}

impl GetTransacSmsReportResponseReportsItemBuilder {
    pub fn accepted(mut self, value: i64) -> Self {
        self.accepted = Some(value);
        self
    }

    pub fn blocked(mut self, value: i64) -> Self {
        self.blocked = Some(value);
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    /// Consumes the builder and constructs a [`GetTransacSmsReportResponseReportsItem`].
    pub fn build(self) -> Result<GetTransacSmsReportResponseReportsItem, BuildError> {
        Ok(GetTransacSmsReportResponseReportsItem {
            accepted: self.accepted,
            blocked: self.blocked,
            date: self.date,
            delivered: self.delivered,
            hard_bounces: self.hard_bounces,
            rejected: self.rejected,
            replied: self.replied,
            requests: self.requests,
            skipped: self.skipped,
            soft_bounces: self.soft_bounces,
            unsubscribed: self.unsubscribed,
        })
    }
}
