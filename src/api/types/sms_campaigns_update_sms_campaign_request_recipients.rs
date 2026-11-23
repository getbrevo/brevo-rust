pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSmsCampaignRequestRecipients {
    /// List ids which have to be excluded from a campaign
    #[serde(rename = "exclusionListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_list_ids: Option<Vec<i64>>,
    /// Lists Ids to send the campaign to. **REQUIRED if scheduledAt is not empty**
    #[serde(rename = "listIds")]
    #[serde(default)]
    pub list_ids: Vec<i64>,
}

impl UpdateSmsCampaignRequestRecipients {
    pub fn builder() -> UpdateSmsCampaignRequestRecipientsBuilder {
        <UpdateSmsCampaignRequestRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSmsCampaignRequestRecipientsBuilder {
    exclusion_list_ids: Option<Vec<i64>>,
    list_ids: Option<Vec<i64>>,
}

impl UpdateSmsCampaignRequestRecipientsBuilder {
    pub fn exclusion_list_ids(mut self, value: Vec<i64>) -> Self {
        self.exclusion_list_ids = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSmsCampaignRequestRecipients`].
    /// This method will fail if any of the following fields are not set:
    /// - [`list_ids`](UpdateSmsCampaignRequestRecipientsBuilder::list_ids)
    pub fn build(self) -> Result<UpdateSmsCampaignRequestRecipients, BuildError> {
        Ok(UpdateSmsCampaignRequestRecipients {
            exclusion_list_ids: self.exclusion_list_ids,
            list_ids: self
                .list_ids
                .ok_or_else(|| BuildError::missing_field("list_ids"))?,
        })
    }
}
