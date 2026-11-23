pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactStatsResponseClickedItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    #[serde(default)]
    pub campaign_id: i64,
    #[serde(default)]
    pub links: Vec<GetContactStatsResponseClickedItemLinksItem>,
}

impl GetContactStatsResponseClickedItem {
    pub fn builder() -> GetContactStatsResponseClickedItemBuilder {
        <GetContactStatsResponseClickedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactStatsResponseClickedItemBuilder {
    campaign_id: Option<i64>,
    links: Option<Vec<GetContactStatsResponseClickedItemLinksItem>>,
}

impl GetContactStatsResponseClickedItemBuilder {
    pub fn campaign_id(mut self, value: i64) -> Self {
        self.campaign_id = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<GetContactStatsResponseClickedItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactStatsResponseClickedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](GetContactStatsResponseClickedItemBuilder::campaign_id)
    /// - [`links`](GetContactStatsResponseClickedItemBuilder::links)
    pub fn build(self) -> Result<GetContactStatsResponseClickedItem, BuildError> {
        Ok(GetContactStatsResponseClickedItem {
            campaign_id: self
                .campaign_id
                .ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
