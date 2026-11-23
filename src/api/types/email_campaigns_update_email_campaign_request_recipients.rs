pub use crate::prelude::*;

/// Segment ids and List ids to include/exclude from campaign
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEmailCampaignRequestRecipients {
    /// List ids which have to be excluded from a campaign
    #[serde(rename = "exclusionListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_list_ids: Option<Vec<i64>>,
    /// Segment ids which have to be excluded from a campaign.
    #[serde(rename = "exclusionSegmentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_segment_ids: Option<Vec<i64>>,
    /// Lists Ids to send the campaign to. **Campaign should only be updated with listIds if listIds were used to create it. REQUIRED if already not present in campaign and scheduledAt is not empty**
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
    /// **Mandatory if listIds are not used. Campaign should only be updated with segmentIds if segmentIds were used to create it.** Segment ids to send the campaign to.
    #[serde(rename = "segmentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_ids: Option<Vec<i64>>,
}

impl UpdateEmailCampaignRequestRecipients {
    pub fn builder() -> UpdateEmailCampaignRequestRecipientsBuilder {
        <UpdateEmailCampaignRequestRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailCampaignRequestRecipientsBuilder {
    exclusion_list_ids: Option<Vec<i64>>,
    exclusion_segment_ids: Option<Vec<i64>>,
    list_ids: Option<Vec<i64>>,
    segment_ids: Option<Vec<i64>>,
}

impl UpdateEmailCampaignRequestRecipientsBuilder {
    pub fn exclusion_list_ids(mut self, value: Vec<i64>) -> Self {
        self.exclusion_list_ids = Some(value);
        self
    }

    pub fn exclusion_segment_ids(mut self, value: Vec<i64>) -> Self {
        self.exclusion_segment_ids = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
        self
    }

    pub fn segment_ids(mut self, value: Vec<i64>) -> Self {
        self.segment_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailCampaignRequestRecipients`].
    pub fn build(self) -> Result<UpdateEmailCampaignRequestRecipients, BuildError> {
        Ok(UpdateEmailCampaignRequestRecipients {
            exclusion_list_ids: self.exclusion_list_ids,
            exclusion_segment_ids: self.exclusion_segment_ids,
            list_ids: self.list_ids,
            segment_ids: self.segment_ids,
        })
    }
}
