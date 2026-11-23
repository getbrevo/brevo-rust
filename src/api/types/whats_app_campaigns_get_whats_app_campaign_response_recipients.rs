pub use crate::prelude::*;

/// Recipients of the WhatsApp Campaign
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppCampaignResponseRecipients {
    /// List of excluded list IDs
    #[serde(rename = "excludedLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_lists: Option<Vec<i64>>,
    /// List of included list IDs
    #[serde(rename = "includedLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_lists: Option<Vec<i64>>,
    /// List of segment IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<i64>>,
    /// Type of recipients (list or segment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetWhatsAppCampaignResponseRecipients {
    pub fn builder() -> GetWhatsAppCampaignResponseRecipientsBuilder {
        <GetWhatsAppCampaignResponseRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppCampaignResponseRecipientsBuilder {
    excluded_lists: Option<Vec<i64>>,
    included_lists: Option<Vec<i64>>,
    segments: Option<Vec<i64>>,
    r#type: Option<String>,
}

impl GetWhatsAppCampaignResponseRecipientsBuilder {
    pub fn excluded_lists(mut self, value: Vec<i64>) -> Self {
        self.excluded_lists = Some(value);
        self
    }

    pub fn included_lists(mut self, value: Vec<i64>) -> Self {
        self.included_lists = Some(value);
        self
    }

    pub fn segments(mut self, value: Vec<i64>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppCampaignResponseRecipients`].
    pub fn build(self) -> Result<GetWhatsAppCampaignResponseRecipients, BuildError> {
        Ok(GetWhatsAppCampaignResponseRecipients {
            excluded_lists: self.excluded_lists,
            included_lists: self.included_lists,
            segments: self.segments,
            r#type: self.r#type,
        })
    }
}
