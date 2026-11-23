pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCampaignRecipients {
    /// Segment IDs excluded from the campaign
    #[serde(rename = "excludedSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<i64>>,
    #[serde(rename = "exclusionLists")]
    #[serde(default)]
    pub exclusion_lists: Vec<i64>,
    #[serde(default)]
    pub lists: Vec<i64>,
    /// Segment IDs included in the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<i64>>,
}

impl GetCampaignRecipients {
    pub fn builder() -> GetCampaignRecipientsBuilder {
        <GetCampaignRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCampaignRecipientsBuilder {
    excluded_segments: Option<Vec<i64>>,
    exclusion_lists: Option<Vec<i64>>,
    lists: Option<Vec<i64>>,
    segments: Option<Vec<i64>>,
}

impl GetCampaignRecipientsBuilder {
    pub fn excluded_segments(mut self, value: Vec<i64>) -> Self {
        self.excluded_segments = Some(value);
        self
    }

    pub fn exclusion_lists(mut self, value: Vec<i64>) -> Self {
        self.exclusion_lists = Some(value);
        self
    }

    pub fn lists(mut self, value: Vec<i64>) -> Self {
        self.lists = Some(value);
        self
    }

    pub fn segments(mut self, value: Vec<i64>) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCampaignRecipients`].
    /// This method will fail if any of the following fields are not set:
    /// - [`exclusion_lists`](GetCampaignRecipientsBuilder::exclusion_lists)
    /// - [`lists`](GetCampaignRecipientsBuilder::lists)
    pub fn build(self) -> Result<GetCampaignRecipients, BuildError> {
        Ok(GetCampaignRecipients {
            excluded_segments: self.excluded_segments,
            exclusion_lists: self
                .exclusion_lists
                .ok_or_else(|| BuildError::missing_field("exclusion_lists"))?,
            lists: self
                .lists
                .ok_or_else(|| BuildError::missing_field("lists"))?,
            segments: self.segments,
        })
    }
}
