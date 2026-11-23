pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDeviceBrowserStats {
    /// Number of total clicks for the campaign using the particular browser
    #[serde(default)]
    pub clickers: i64,
    /// Number of unique clicks for the campaign using the particular browser
    #[serde(rename = "uniqueClicks")]
    #[serde(default)]
    pub unique_clicks: i64,
    /// Number of unique openings for the campaign using the particular browser
    #[serde(rename = "uniqueViews")]
    #[serde(default)]
    pub unique_views: i64,
    /// Number of openings for the campaign using the particular browser
    #[serde(default)]
    pub viewed: i64,
}

impl GetDeviceBrowserStats {
    pub fn builder() -> GetDeviceBrowserStatsBuilder {
        <GetDeviceBrowserStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDeviceBrowserStatsBuilder {
    clickers: Option<i64>,
    unique_clicks: Option<i64>,
    unique_views: Option<i64>,
    viewed: Option<i64>,
}

impl GetDeviceBrowserStatsBuilder {
    pub fn clickers(mut self, value: i64) -> Self {
        self.clickers = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: i64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    pub fn unique_views(mut self, value: i64) -> Self {
        self.unique_views = Some(value);
        self
    }

    pub fn viewed(mut self, value: i64) -> Self {
        self.viewed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDeviceBrowserStats`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clickers`](GetDeviceBrowserStatsBuilder::clickers)
    /// - [`unique_clicks`](GetDeviceBrowserStatsBuilder::unique_clicks)
    /// - [`unique_views`](GetDeviceBrowserStatsBuilder::unique_views)
    /// - [`viewed`](GetDeviceBrowserStatsBuilder::viewed)
    pub fn build(self) -> Result<GetDeviceBrowserStats, BuildError> {
        Ok(GetDeviceBrowserStats {
            clickers: self
                .clickers
                .ok_or_else(|| BuildError::missing_field("clickers"))?,
            unique_clicks: self
                .unique_clicks
                .ok_or_else(|| BuildError::missing_field("unique_clicks"))?,
            unique_views: self
                .unique_views
                .ok_or_else(|| BuildError::missing_field("unique_views"))?,
            viewed: self
                .viewed
                .ok_or_else(|| BuildError::missing_field("viewed"))?,
        })
    }
}
