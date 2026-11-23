pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetListResponseCampaignStatsItem {
    /// ID of the campaign
    #[serde(rename = "campaignId")]
    #[serde(default)]
    pub campaign_id: i64,
    #[serde(default)]
    pub stats: GetCampaignStats,
}

impl GetListResponseCampaignStatsItem {
    pub fn builder() -> GetListResponseCampaignStatsItemBuilder {
        <GetListResponseCampaignStatsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetListResponseCampaignStatsItemBuilder {
    campaign_id: Option<i64>,
    stats: Option<GetCampaignStats>,
}

impl GetListResponseCampaignStatsItemBuilder {
    pub fn campaign_id(mut self, value: i64) -> Self {
        self.campaign_id = Some(value);
        self
    }

    pub fn stats(mut self, value: GetCampaignStats) -> Self {
        self.stats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetListResponseCampaignStatsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](GetListResponseCampaignStatsItemBuilder::campaign_id)
    /// - [`stats`](GetListResponseCampaignStatsItemBuilder::stats)
    pub fn build(self) -> Result<GetListResponseCampaignStatsItem, BuildError> {
        Ok(GetListResponseCampaignStatsItem {
            campaign_id: self
                .campaign_id
                .ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            stats: self
                .stats
                .ok_or_else(|| BuildError::missing_field("stats"))?,
        })
    }
}
