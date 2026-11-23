pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<i64>,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    #[serde(default)]
    pub event_time: String,
    /// IP from which the user has unsubscribed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem {
    pub fn builder() -> GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItemBuilder {
        <GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItemBuilder {
    campaign_id: Option<i64>,
    event_time: Option<String>,
    ip: Option<String>,
}

impl GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItemBuilder {
    pub fn campaign_id(mut self, value: i64) -> Self {
        self.campaign_id = Some(value);
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

    /// Consumes the builder and constructs a [`GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_time`](GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItemBuilder::event_time)
    pub fn build(
        self,
    ) -> Result<GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem, BuildError> {
        Ok(
            GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem {
                campaign_id: self.campaign_id,
                event_time: self
                    .event_time
                    .ok_or_else(|| BuildError::missing_field("event_time"))?,
                ip: self.ip,
            },
        )
    }
}
