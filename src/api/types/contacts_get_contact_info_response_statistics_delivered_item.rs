pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsDeliveredItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    #[serde(default)]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    #[serde(default)]
    pub event_time: String,
}

impl GetContactInfoResponseStatisticsDeliveredItem {
    pub fn builder() -> GetContactInfoResponseStatisticsDeliveredItemBuilder {
        <GetContactInfoResponseStatisticsDeliveredItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsDeliveredItemBuilder {
    campaign_id: Option<i64>,
    event_time: Option<String>,
}

impl GetContactInfoResponseStatisticsDeliveredItemBuilder {
    pub fn campaign_id(mut self, value: i64) -> Self {
        self.campaign_id = Some(value);
        self
    }

    pub fn event_time(mut self, value: impl Into<String>) -> Self {
        self.event_time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsDeliveredItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](GetContactInfoResponseStatisticsDeliveredItemBuilder::campaign_id)
    /// - [`event_time`](GetContactInfoResponseStatisticsDeliveredItemBuilder::event_time)
    pub fn build(self) -> Result<GetContactInfoResponseStatisticsDeliveredItem, BuildError> {
        Ok(GetContactInfoResponseStatisticsDeliveredItem {
            campaign_id: self
                .campaign_id
                .ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
        })
    }
}
