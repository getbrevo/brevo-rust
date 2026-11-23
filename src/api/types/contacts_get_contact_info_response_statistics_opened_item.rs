pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsOpenedItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    #[serde(default)]
    pub campaign_id: i64,
    /// Number of openings for the campaign
    #[serde(default)]
    pub count: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    #[serde(default)]
    pub event_time: String,
    /// IP from which the user has opened the email
    #[serde(default)]
    pub ip: String,
}

impl GetContactInfoResponseStatisticsOpenedItem {
    pub fn builder() -> GetContactInfoResponseStatisticsOpenedItemBuilder {
        <GetContactInfoResponseStatisticsOpenedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsOpenedItemBuilder {
    campaign_id: Option<i64>,
    count: Option<i64>,
    event_time: Option<String>,
    ip: Option<String>,
}

impl GetContactInfoResponseStatisticsOpenedItemBuilder {
    pub fn campaign_id(mut self, value: i64) -> Self {
        self.campaign_id = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn event_time(mut self, value: impl Into<String>) -> Self {
        self.event_time = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsOpenedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](GetContactInfoResponseStatisticsOpenedItemBuilder::campaign_id)
    /// - [`count`](GetContactInfoResponseStatisticsOpenedItemBuilder::count)
    /// - [`event_time`](GetContactInfoResponseStatisticsOpenedItemBuilder::event_time)
    /// - [`ip`](GetContactInfoResponseStatisticsOpenedItemBuilder::ip)
    pub fn build(self) -> Result<GetContactInfoResponseStatisticsOpenedItem, BuildError> {
        Ok(GetContactInfoResponseStatisticsOpenedItem {
            campaign_id: self
                .campaign_id
                .ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
        })
    }
}
