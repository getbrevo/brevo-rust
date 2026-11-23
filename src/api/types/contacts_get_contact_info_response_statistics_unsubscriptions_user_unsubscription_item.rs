pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem {
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

impl GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem {
    pub fn builder() -> GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItemBuilder
    {
        <GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItemBuilder {
    campaign_id: Option<i64>,
    event_time: Option<String>,
    ip: Option<String>,
}

impl GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItemBuilder {
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

    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_time`](GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItemBuilder::event_time)
    pub fn build(
        self,
    ) -> Result<GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem, BuildError>
    {
        Ok(
            GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem {
                campaign_id: self.campaign_id,
                event_time: self
                    .event_time
                    .ok_or_else(|| BuildError::missing_field("event_time"))?,
                ip: self.ip,
            },
        )
    }
}
