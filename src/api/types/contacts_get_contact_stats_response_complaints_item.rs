pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactStatsResponseComplaintsItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    #[serde(default)]
    pub campaign_id: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    #[serde(default)]
    pub event_time: String,
}

impl GetContactStatsResponseComplaintsItem {
    pub fn builder() -> GetContactStatsResponseComplaintsItemBuilder {
        <GetContactStatsResponseComplaintsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactStatsResponseComplaintsItemBuilder {
    campaign_id: Option<i64>,
    event_time: Option<String>,
}

impl GetContactStatsResponseComplaintsItemBuilder {
    pub fn campaign_id(mut self, value: i64) -> Self {
        self.campaign_id = Some(value);
        self
    }

    pub fn event_time(mut self, value: impl Into<String>) -> Self {
        self.event_time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactStatsResponseComplaintsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](GetContactStatsResponseComplaintsItemBuilder::campaign_id)
    /// - [`event_time`](GetContactStatsResponseComplaintsItemBuilder::event_time)
    pub fn build(self) -> Result<GetContactStatsResponseComplaintsItem, BuildError> {
        Ok(GetContactStatsResponseComplaintsItem {
            campaign_id: self
                .campaign_id
                .ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
        })
    }
}
