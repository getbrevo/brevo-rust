pub use crate::prelude::*;

/// Segment ids and List ids to include/exclude from campaign
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWhatsAppCampaignRequestRecipients {
    /// List ids to exclude from the campaign
    #[serde(rename = "excludedListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_list_ids: Option<Vec<i64>>,
    /// **Mandatory if scheduledAt is not empty**. List Ids to send the campaign to
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
    /// **Mandatory if listIds are not used**. Segment ids to send the campaign to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<i64>>,
}

impl UpdateWhatsAppCampaignRequestRecipients {
    pub fn builder() -> UpdateWhatsAppCampaignRequestRecipientsBuilder {
        <UpdateWhatsAppCampaignRequestRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWhatsAppCampaignRequestRecipientsBuilder {
    excluded_list_ids: Option<Vec<i64>>,
    list_ids: Option<Vec<i64>>,
    segments: Option<Vec<i64>>,
}

impl UpdateWhatsAppCampaignRequestRecipientsBuilder {
    pub fn excluded_list_ids(mut self, value: Vec<i64>) -> Self {
        self.excluded_list_ids = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
        self
    }

    pub fn segments(mut self, value: Vec<i64>) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWhatsAppCampaignRequestRecipients`].
    pub fn build(self) -> Result<UpdateWhatsAppCampaignRequestRecipients, BuildError> {
        Ok(UpdateWhatsAppCampaignRequestRecipients {
            excluded_list_ids: self.excluded_list_ids,
            list_ids: self.list_ids,
            segments: self.segments,
        })
    }
}
