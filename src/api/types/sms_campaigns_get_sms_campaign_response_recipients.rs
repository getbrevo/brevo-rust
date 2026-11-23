pub use crate::prelude::*;

/// Recipients of the SMS campaign. For a single campaign, lists and exclusion lists are returned as objects with id and name.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsCampaignResponseRecipients {
    #[serde(rename = "exclusionLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_lists: Option<Vec<GetSmsCampaignResponseRecipientsExclusionListsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<GetSmsCampaignResponseRecipientsListsItem>>,
}

impl GetSmsCampaignResponseRecipients {
    pub fn builder() -> GetSmsCampaignResponseRecipientsBuilder {
        <GetSmsCampaignResponseRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsCampaignResponseRecipientsBuilder {
    exclusion_lists: Option<Vec<GetSmsCampaignResponseRecipientsExclusionListsItem>>,
    lists: Option<Vec<GetSmsCampaignResponseRecipientsListsItem>>,
}

impl GetSmsCampaignResponseRecipientsBuilder {
    pub fn exclusion_lists(
        mut self,
        value: Vec<GetSmsCampaignResponseRecipientsExclusionListsItem>,
    ) -> Self {
        self.exclusion_lists = Some(value);
        self
    }

    pub fn lists(mut self, value: Vec<GetSmsCampaignResponseRecipientsListsItem>) -> Self {
        self.lists = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsCampaignResponseRecipients`].
    pub fn build(self) -> Result<GetSmsCampaignResponseRecipients, BuildError> {
        Ok(GetSmsCampaignResponseRecipients {
            exclusion_lists: self.exclusion_lists,
            lists: self.lists,
        })
    }
}
