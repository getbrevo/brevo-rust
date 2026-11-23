pub use crate::prelude::*;

/// Segment ids and List ids to include/exclude from campaign
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEmailCampaignRequestRecipients {
    /// List ids to exclude from the campaign
    #[serde(rename = "exclusionListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_list_ids: Option<Vec<i64>>,
    /// Segment ids which have to be excluded from a campaign.
    #[serde(rename = "exclusionSegmentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_segment_ids: Option<Vec<i64>>,
    /// **Mandatory if scheduledAt is not empty**. List Ids to send the campaign to
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
    /// **Mandatory if listIds are not used**. Segment ids to send the campaign to.
    #[serde(rename = "segmentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_ids: Option<Vec<i64>>,
}

impl CreateEmailCampaignRequestRecipients {
    pub fn builder() -> CreateEmailCampaignRequestRecipientsBuilder {
        <CreateEmailCampaignRequestRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEmailCampaignRequestRecipientsBuilder {
    exclusion_list_ids: Option<Vec<i64>>,
    exclusion_segment_ids: Option<Vec<i64>>,
    list_ids: Option<Vec<i64>>,
    segment_ids: Option<Vec<i64>>,
}

impl CreateEmailCampaignRequestRecipientsBuilder {
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

    /// Consumes the builder and constructs a [`CreateEmailCampaignRequestRecipients`].
    pub fn build(self) -> Result<CreateEmailCampaignRequestRecipients, BuildError> {
        Ok(CreateEmailCampaignRequestRecipients {
            exclusion_list_ids: self.exclusion_list_ids,
            exclusion_segment_ids: self.exclusion_segment_ids,
            list_ids: self.list_ids,
            segment_ids: self.segment_ids,
        })
    }
}
