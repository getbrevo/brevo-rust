pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmtpReportResponseReportsItem {
    /// Number of blocked emails for the date
    #[serde(default)]
    pub blocked: i64,
    /// Number of clicks for the date
    #[serde(default)]
    pub clicks: i64,
    /// Date of the statistics
    #[serde(default)]
    pub date: NaiveDate,
    /// Number of delivered emails for the date
    #[serde(default)]
    pub delivered: i64,
    /// Number of hardbounces for the date
    #[serde(rename = "hardBounces")]
    #[serde(default)]
    pub hard_bounces: i64,
    /// Number of invalid emails for the date
    #[serde(default)]
    pub invalid: i64,
    /// Number of openings for the date
    #[serde(default)]
    pub opens: i64,
    /// Number of requests for the date
    #[serde(default)]
    pub requests: i64,
    /// Number of softbounces for the date
    #[serde(rename = "softBounces")]
    #[serde(default)]
    pub soft_bounces: i64,
    /// Number of complaints (spam reports) for the date
    #[serde(rename = "spamReports")]
    #[serde(default)]
    pub spam_reports: i64,
    /// Number of unique clicks for the date
    #[serde(rename = "uniqueClicks")]
    #[serde(default)]
    pub unique_clicks: i64,
    /// Number of unique openings for the date
    #[serde(rename = "uniqueOpens")]
    #[serde(default)]
    pub unique_opens: i64,
    /// Number of unsubscribed emails for the date
    #[serde(default)]
    pub unsubscribed: i64,
}

impl GetSmtpReportResponseReportsItem {
    pub fn builder() -> GetSmtpReportResponseReportsItemBuilder {
        <GetSmtpReportResponseReportsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmtpReportResponseReportsItemBuilder {
    blocked: Option<i64>,
    clicks: Option<i64>,
    date: Option<NaiveDate>,
    delivered: Option<i64>,
    hard_bounces: Option<i64>,
    invalid: Option<i64>,
    opens: Option<i64>,
    requests: Option<i64>,
    soft_bounces: Option<i64>,
    spam_reports: Option<i64>,
    unique_clicks: Option<i64>,
    unique_opens: Option<i64>,
    unsubscribed: Option<i64>,
}

impl GetSmtpReportResponseReportsItemBuilder {
    pub fn blocked(mut self, value: i64) -> Self {
        self.blocked = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
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

    pub fn invalid(mut self, value: i64) -> Self {
        self.invalid = Some(value);
        self
    }

    pub fn opens(mut self, value: i64) -> Self {
        self.opens = Some(value);
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

    /// Consumes the builder and constructs a [`GetSmtpReportResponseReportsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`blocked`](GetSmtpReportResponseReportsItemBuilder::blocked)
    /// - [`clicks`](GetSmtpReportResponseReportsItemBuilder::clicks)
    /// - [`date`](GetSmtpReportResponseReportsItemBuilder::date)
    /// - [`delivered`](GetSmtpReportResponseReportsItemBuilder::delivered)
    /// - [`hard_bounces`](GetSmtpReportResponseReportsItemBuilder::hard_bounces)
    /// - [`invalid`](GetSmtpReportResponseReportsItemBuilder::invalid)
    /// - [`opens`](GetSmtpReportResponseReportsItemBuilder::opens)
    /// - [`requests`](GetSmtpReportResponseReportsItemBuilder::requests)
    /// - [`soft_bounces`](GetSmtpReportResponseReportsItemBuilder::soft_bounces)
    /// - [`spam_reports`](GetSmtpReportResponseReportsItemBuilder::spam_reports)
    /// - [`unique_clicks`](GetSmtpReportResponseReportsItemBuilder::unique_clicks)
    /// - [`unique_opens`](GetSmtpReportResponseReportsItemBuilder::unique_opens)
    /// - [`unsubscribed`](GetSmtpReportResponseReportsItemBuilder::unsubscribed)
    pub fn build(self) -> Result<GetSmtpReportResponseReportsItem, BuildError> {
        Ok(GetSmtpReportResponseReportsItem {
            blocked: self
                .blocked
                .ok_or_else(|| BuildError::missing_field("blocked"))?,
            clicks: self
                .clicks
                .ok_or_else(|| BuildError::missing_field("clicks"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            delivered: self
                .delivered
                .ok_or_else(|| BuildError::missing_field("delivered"))?,
            hard_bounces: self
                .hard_bounces
                .ok_or_else(|| BuildError::missing_field("hard_bounces"))?,
            invalid: self
                .invalid
                .ok_or_else(|| BuildError::missing_field("invalid"))?,
            opens: self
                .opens
                .ok_or_else(|| BuildError::missing_field("opens"))?,
            requests: self
                .requests
                .ok_or_else(|| BuildError::missing_field("requests"))?,
            soft_bounces: self
                .soft_bounces
                .ok_or_else(|| BuildError::missing_field("soft_bounces"))?,
            spam_reports: self
                .spam_reports
                .ok_or_else(|| BuildError::missing_field("spam_reports"))?,
            unique_clicks: self
                .unique_clicks
                .ok_or_else(|| BuildError::missing_field("unique_clicks"))?,
            unique_opens: self
                .unique_opens
                .ok_or_else(|| BuildError::missing_field("unique_opens"))?,
            unsubscribed: self
                .unsubscribed
                .ok_or_else(|| BuildError::missing_field("unsubscribed"))?,
        })
    }
}
