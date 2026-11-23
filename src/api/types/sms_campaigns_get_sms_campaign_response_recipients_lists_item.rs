pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsCampaignResponseRecipientsListsItem {
    /// ID of the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Name of the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetSmsCampaignResponseRecipientsListsItem {
    pub fn builder() -> GetSmsCampaignResponseRecipientsListsItemBuilder {
        <GetSmsCampaignResponseRecipientsListsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsCampaignResponseRecipientsListsItemBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl GetSmsCampaignResponseRecipientsListsItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSmsCampaignResponseRecipientsListsItem`].
    pub fn build(self) -> Result<GetSmsCampaignResponseRecipientsListsItem, BuildError> {
        Ok(GetSmsCampaignResponseRecipientsListsItem {
            id: self.id,
            name: self.name,
        })
    }
}
