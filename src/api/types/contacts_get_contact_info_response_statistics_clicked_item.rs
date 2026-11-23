pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsClickedItem {
    /// ID of the campaign which generated the event
    #[serde(rename = "campaignId")]
    #[serde(default)]
    pub campaign_id: i64,
    /// Listing of the clicked links for the campaign
    #[serde(default)]
    pub links: Vec<GetContactInfoResponseStatisticsClickedItemLinksItem>,
}

impl GetContactInfoResponseStatisticsClickedItem {
    pub fn builder() -> GetContactInfoResponseStatisticsClickedItemBuilder {
        <GetContactInfoResponseStatisticsClickedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsClickedItemBuilder {
    campaign_id: Option<i64>,
    links: Option<Vec<GetContactInfoResponseStatisticsClickedItemLinksItem>>,
}

impl GetContactInfoResponseStatisticsClickedItemBuilder {
    pub fn campaign_id(mut self, value: i64) -> Self {
        self.campaign_id = Some(value);
        self
    }

    pub fn links(
        mut self,
        value: Vec<GetContactInfoResponseStatisticsClickedItemLinksItem>,
    ) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsClickedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](GetContactInfoResponseStatisticsClickedItemBuilder::campaign_id)
    /// - [`links`](GetContactInfoResponseStatisticsClickedItemBuilder::links)
    pub fn build(self) -> Result<GetContactInfoResponseStatisticsClickedItem, BuildError> {
        Ok(GetContactInfoResponseStatisticsClickedItem {
            campaign_id: self
                .campaign_id
                .ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
