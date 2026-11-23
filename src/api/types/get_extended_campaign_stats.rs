pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetExtendedCampaignStats {
    /// List-wise statistics of the campaign.
    #[serde(rename = "campaignStats")]
    #[serde(default)]
    pub campaign_stats: Vec<GetCampaignStats>,
    /// Overall statistics of the campaign
    #[serde(rename = "globalStats")]
    #[serde(default)]
    pub global_stats: GetCampaignStats,
    /// Statistics about the number of clicks for each link in the campaign. Keys are the link URLs, values are click counts. Only populated when the `statistics` query parameter is set to `linksStats`.
    #[serde(rename = "linksStats")]
    #[serde(default)]
    pub links_stats: HashMap<String, i64>,
    /// Number of clicks on mirror link
    #[serde(rename = "mirrorClick")]
    #[serde(default)]
    pub mirror_click: i64,
    /// Number of remaining emails to send
    #[serde(default)]
    pub remaining: i64,
    /// Statistics of the campaign grouped by browser. Only available when retrieving a single campaign with the `statistics` query parameter set to `statsByBrowser`.
    #[serde(rename = "statsByBrowser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_by_browser: Option<HashMap<String, GetDeviceBrowserStats>>,
    #[serde(rename = "statsByDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_by_device: Option<GetExtendedCampaignStatsStatsByDevice>,
    /// Statistics of the campaign grouped by email domain. Only populated when the `statistics` query parameter is set to `statsByDomain`.
    #[serde(rename = "statsByDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_by_domain: Option<HashMap<String, GetCampaignStats>>,
}

impl GetExtendedCampaignStats {
    pub fn builder() -> GetExtendedCampaignStatsBuilder {
        <GetExtendedCampaignStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetExtendedCampaignStatsBuilder {
    campaign_stats: Option<Vec<GetCampaignStats>>,
    global_stats: Option<GetCampaignStats>,
    links_stats: Option<HashMap<String, i64>>,
    mirror_click: Option<i64>,
    remaining: Option<i64>,
    stats_by_browser: Option<HashMap<String, GetDeviceBrowserStats>>,
    stats_by_device: Option<GetExtendedCampaignStatsStatsByDevice>,
    stats_by_domain: Option<HashMap<String, GetCampaignStats>>,
}

impl GetExtendedCampaignStatsBuilder {
    pub fn campaign_stats(mut self, value: Vec<GetCampaignStats>) -> Self {
        self.campaign_stats = Some(value);
        self
    }

    pub fn global_stats(mut self, value: GetCampaignStats) -> Self {
        self.global_stats = Some(value);
        self
    }

    pub fn links_stats(mut self, value: HashMap<String, i64>) -> Self {
        self.links_stats = Some(value);
        self
    }

    pub fn mirror_click(mut self, value: i64) -> Self {
        self.mirror_click = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    pub fn stats_by_browser(mut self, value: HashMap<String, GetDeviceBrowserStats>) -> Self {
        self.stats_by_browser = Some(value);
        self
    }

    pub fn stats_by_device(mut self, value: GetExtendedCampaignStatsStatsByDevice) -> Self {
        self.stats_by_device = Some(value);
        self
    }

    pub fn stats_by_domain(mut self, value: HashMap<String, GetCampaignStats>) -> Self {
        self.stats_by_domain = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetExtendedCampaignStats`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_stats`](GetExtendedCampaignStatsBuilder::campaign_stats)
    /// - [`global_stats`](GetExtendedCampaignStatsBuilder::global_stats)
    /// - [`links_stats`](GetExtendedCampaignStatsBuilder::links_stats)
    /// - [`mirror_click`](GetExtendedCampaignStatsBuilder::mirror_click)
    /// - [`remaining`](GetExtendedCampaignStatsBuilder::remaining)
    pub fn build(self) -> Result<GetExtendedCampaignStats, BuildError> {
        Ok(GetExtendedCampaignStats {
            campaign_stats: self
                .campaign_stats
                .ok_or_else(|| BuildError::missing_field("campaign_stats"))?,
            global_stats: self
                .global_stats
                .ok_or_else(|| BuildError::missing_field("global_stats"))?,
            links_stats: self
                .links_stats
                .ok_or_else(|| BuildError::missing_field("links_stats"))?,
            mirror_click: self
                .mirror_click
                .ok_or_else(|| BuildError::missing_field("mirror_click"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            stats_by_browser: self.stats_by_browser,
            stats_by_device: self.stats_by_device,
            stats_by_domain: self.stats_by_domain,
        })
    }
}
