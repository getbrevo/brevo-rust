pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsCampaignStats {
    /// Number of replies to the SMS
    #[serde(default)]
    pub answered: i64,
    /// Number of delivered SMS
    #[serde(default)]
    pub delivered: i64,
    /// Number of hardbounced SMS
    #[serde(rename = "hardBounces")]
    #[serde(default)]
    pub hard_bounces: i64,
    /// Number of processing SMS
    #[serde(default)]
    pub processing: i64,
    /// Number of sent SMS
    #[serde(default)]
    pub sent: i64,
    /// Number of softbounced SMS
    #[serde(rename = "softBounces")]
    #[serde(default)]
    pub soft_bounces: i64,
    /// Number of unsubscription SMS
    #[serde(default)]
    pub unsubscriptions: i64,
}

impl GetSmsCampaignStats {
    pub fn builder() -> GetSmsCampaignStatsBuilder {
        <GetSmsCampaignStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsCampaignStatsBuilder {
    answered: Option<i64>,
    delivered: Option<i64>,
    hard_bounces: Option<i64>,
    processing: Option<i64>,
    sent: Option<i64>,
    soft_bounces: Option<i64>,
    unsubscriptions: Option<i64>,
}

impl GetSmsCampaignStatsBuilder {
    pub fn answered(mut self, value: i64) -> Self {
        self.answered = Some(value);
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

    pub fn processing(mut self, value: i64) -> Self {
        self.processing = Some(value);
        self
    }

    pub fn sent(mut self, value: i64) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn soft_bounces(mut self, value: i64) -> Self {
        self.soft_bounces = Some(value);
        self
    }

    pub fn unsubscriptions(mut self, value: i64) -> Self {
        self.unsubscriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsCampaignStats`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answered`](GetSmsCampaignStatsBuilder::answered)
    /// - [`delivered`](GetSmsCampaignStatsBuilder::delivered)
    /// - [`hard_bounces`](GetSmsCampaignStatsBuilder::hard_bounces)
    /// - [`processing`](GetSmsCampaignStatsBuilder::processing)
    /// - [`sent`](GetSmsCampaignStatsBuilder::sent)
    /// - [`soft_bounces`](GetSmsCampaignStatsBuilder::soft_bounces)
    /// - [`unsubscriptions`](GetSmsCampaignStatsBuilder::unsubscriptions)
    pub fn build(self) -> Result<GetSmsCampaignStats, BuildError> {
        Ok(GetSmsCampaignStats {
            answered: self
                .answered
                .ok_or_else(|| BuildError::missing_field("answered"))?,
            delivered: self
                .delivered
                .ok_or_else(|| BuildError::missing_field("delivered"))?,
            hard_bounces: self
                .hard_bounces
                .ok_or_else(|| BuildError::missing_field("hard_bounces"))?,
            processing: self
                .processing
                .ok_or_else(|| BuildError::missing_field("processing"))?,
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            soft_bounces: self
                .soft_bounces
                .ok_or_else(|| BuildError::missing_field("soft_bounces"))?,
            unsubscriptions: self
                .unsubscriptions
                .ok_or_else(|| BuildError::missing_field("unsubscriptions"))?,
        })
    }
}
