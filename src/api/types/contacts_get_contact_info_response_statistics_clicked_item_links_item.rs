pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsClickedItemLinksItem {
    /// Number of clicks on this link for the campaign
    #[serde(default)]
    pub count: i64,
    /// UTC date-time of the event
    #[serde(rename = "eventTime")]
    #[serde(default)]
    pub event_time: String,
    /// IP from which the user has clicked on the link
    #[serde(default)]
    pub ip: String,
    /// URL of the clicked link
    #[serde(default)]
    pub url: String,
}

impl GetContactInfoResponseStatisticsClickedItemLinksItem {
    pub fn builder() -> GetContactInfoResponseStatisticsClickedItemLinksItemBuilder {
        <GetContactInfoResponseStatisticsClickedItemLinksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsClickedItemLinksItemBuilder {
    count: Option<i64>,
    event_time: Option<String>,
    ip: Option<String>,
    url: Option<String>,
}

impl GetContactInfoResponseStatisticsClickedItemLinksItemBuilder {
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

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsClickedItemLinksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](GetContactInfoResponseStatisticsClickedItemLinksItemBuilder::count)
    /// - [`event_time`](GetContactInfoResponseStatisticsClickedItemLinksItemBuilder::event_time)
    /// - [`ip`](GetContactInfoResponseStatisticsClickedItemLinksItemBuilder::ip)
    /// - [`url`](GetContactInfoResponseStatisticsClickedItemLinksItemBuilder::url)
    pub fn build(self) -> Result<GetContactInfoResponseStatisticsClickedItemLinksItem, BuildError> {
        Ok(GetContactInfoResponseStatisticsClickedItemLinksItem {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
