pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem {
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    #[serde(default)]
    pub event_time: String,
    /// IP from which the user has been unsubscribed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem {
    pub fn builder() -> GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItemBuilder
    {
        <GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItemBuilder {
    event_time: Option<String>,
    ip: Option<String>,
}

impl GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItemBuilder {
    pub fn event_time(mut self, value: impl Into<String>) -> Self {
        self.event_time = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_time`](GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItemBuilder::event_time)
    pub fn build(
        self,
    ) -> Result<GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem, BuildError>
    {
        Ok(
            GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem {
                event_time: self
                    .event_time
                    .ok_or_else(|| BuildError::missing_field("event_time"))?,
                ip: self.ip,
            },
        )
    }
}
