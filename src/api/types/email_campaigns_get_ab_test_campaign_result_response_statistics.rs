pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAbTestCampaignResultResponseStatistics {
    #[serde(default)]
    pub clicks: AbTestVersionStats,
    #[serde(default)]
    pub complaints: AbTestVersionStats,
    #[serde(rename = "hardBounces")]
    #[serde(default)]
    pub hard_bounces: AbTestVersionStats,
    #[serde(default)]
    pub openers: AbTestVersionStats,
    #[serde(rename = "softBounces")]
    #[serde(default)]
    pub soft_bounces: AbTestVersionStats,
    #[serde(default)]
    pub unsubscribed: AbTestVersionStats,
}

impl GetAbTestCampaignResultResponseStatistics {
    pub fn builder() -> GetAbTestCampaignResultResponseStatisticsBuilder {
        <GetAbTestCampaignResultResponseStatisticsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAbTestCampaignResultResponseStatisticsBuilder {
    clicks: Option<AbTestVersionStats>,
    complaints: Option<AbTestVersionStats>,
    hard_bounces: Option<AbTestVersionStats>,
    openers: Option<AbTestVersionStats>,
    soft_bounces: Option<AbTestVersionStats>,
    unsubscribed: Option<AbTestVersionStats>,
}

impl GetAbTestCampaignResultResponseStatisticsBuilder {
    pub fn clicks(mut self, value: AbTestVersionStats) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn complaints(mut self, value: AbTestVersionStats) -> Self {
        self.complaints = Some(value);
        self
    }

    pub fn hard_bounces(mut self, value: AbTestVersionStats) -> Self {
        self.hard_bounces = Some(value);
        self
    }

    pub fn openers(mut self, value: AbTestVersionStats) -> Self {
        self.openers = Some(value);
        self
    }

    pub fn soft_bounces(mut self, value: AbTestVersionStats) -> Self {
        self.soft_bounces = Some(value);
        self
    }

    pub fn unsubscribed(mut self, value: AbTestVersionStats) -> Self {
        self.unsubscribed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAbTestCampaignResultResponseStatistics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clicks`](GetAbTestCampaignResultResponseStatisticsBuilder::clicks)
    /// - [`complaints`](GetAbTestCampaignResultResponseStatisticsBuilder::complaints)
    /// - [`hard_bounces`](GetAbTestCampaignResultResponseStatisticsBuilder::hard_bounces)
    /// - [`openers`](GetAbTestCampaignResultResponseStatisticsBuilder::openers)
    /// - [`soft_bounces`](GetAbTestCampaignResultResponseStatisticsBuilder::soft_bounces)
    /// - [`unsubscribed`](GetAbTestCampaignResultResponseStatisticsBuilder::unsubscribed)
    pub fn build(self) -> Result<GetAbTestCampaignResultResponseStatistics, BuildError> {
        Ok(GetAbTestCampaignResultResponseStatistics {
            clicks: self
                .clicks
                .ok_or_else(|| BuildError::missing_field("clicks"))?,
            complaints: self
                .complaints
                .ok_or_else(|| BuildError::missing_field("complaints"))?,
            hard_bounces: self
                .hard_bounces
                .ok_or_else(|| BuildError::missing_field("hard_bounces"))?,
            openers: self
                .openers
                .ok_or_else(|| BuildError::missing_field("openers"))?,
            soft_bounces: self
                .soft_bounces
                .ok_or_else(|| BuildError::missing_field("soft_bounces"))?,
            unsubscribed: self
                .unsubscribed
                .ok_or_else(|| BuildError::missing_field("unsubscribed"))?,
        })
    }
}
