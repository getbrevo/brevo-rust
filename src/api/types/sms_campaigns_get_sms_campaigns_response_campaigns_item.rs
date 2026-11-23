pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetSmsCampaignsResponseCampaignsItem {
    /// Content of the SMS Campaign
    #[serde(default)]
    pub content: String,
    /// Creation UTC date-time of the SMS campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// ID of the SMS Campaign
    #[serde(default)]
    pub id: i64,
    /// UTC date-time of last modification of the SMS campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Name of the SMS Campaign
    #[serde(default)]
    pub name: String,
    /// A recognizable prefix added as the Brand Name before the message content. Empty string if not set.
    #[serde(rename = "organisationPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation_prefix: Option<String>,
    /// UTC date-time on which SMS campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format. Empty string if not scheduled.
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Sender of the SMS Campaign
    #[serde(default)]
    pub sender: String,
    /// UTC date-time on which the SMS campaign was sent (YYYY-MM-DDTHH:mm:ss.SSSZ). Only available if the campaign status is 'sent'.
    #[serde(rename = "sentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_date: Option<String>,
    /// Status of the SMS Campaign
    pub status: GetSmsCampaignsResponseCampaignsItemStatus,
    /// Instructions to unsubscribe from future communications. Empty string if not set.
    #[serde(rename = "unsubscribeInstruction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_instruction: Option<String>,
    #[serde(default)]
    pub recipients: GetCampaignRecipients,
    #[serde(default)]
    pub statistics: GetSmsCampaignStats,
}

impl GetSmsCampaignsResponseCampaignsItem {
    pub fn builder() -> GetSmsCampaignsResponseCampaignsItemBuilder {
        <GetSmsCampaignsResponseCampaignsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsCampaignsResponseCampaignsItemBuilder {
    content: Option<String>,
    created_at: Option<String>,
    id: Option<i64>,
    modified_at: Option<String>,
    name: Option<String>,
    organisation_prefix: Option<String>,
    scheduled_at: Option<String>,
    sender: Option<String>,
    sent_date: Option<String>,
    status: Option<GetSmsCampaignsResponseCampaignsItemStatus>,
    unsubscribe_instruction: Option<String>,
    recipients: Option<GetCampaignRecipients>,
    statistics: Option<GetSmsCampaignStats>,
}

impl GetSmsCampaignsResponseCampaignsItemBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organisation_prefix(mut self, value: impl Into<String>) -> Self {
        self.organisation_prefix = Some(value.into());
        self
    }

    pub fn scheduled_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_at = Some(value.into());
        self
    }

    pub fn sender(mut self, value: impl Into<String>) -> Self {
        self.sender = Some(value.into());
        self
    }

    pub fn sent_date(mut self, value: impl Into<String>) -> Self {
        self.sent_date = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetSmsCampaignsResponseCampaignsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn unsubscribe_instruction(mut self, value: impl Into<String>) -> Self {
        self.unsubscribe_instruction = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: GetCampaignRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn statistics(mut self, value: GetSmsCampaignStats) -> Self {
        self.statistics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsCampaignsResponseCampaignsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](GetSmsCampaignsResponseCampaignsItemBuilder::content)
    /// - [`created_at`](GetSmsCampaignsResponseCampaignsItemBuilder::created_at)
    /// - [`id`](GetSmsCampaignsResponseCampaignsItemBuilder::id)
    /// - [`modified_at`](GetSmsCampaignsResponseCampaignsItemBuilder::modified_at)
    /// - [`name`](GetSmsCampaignsResponseCampaignsItemBuilder::name)
    /// - [`sender`](GetSmsCampaignsResponseCampaignsItemBuilder::sender)
    /// - [`status`](GetSmsCampaignsResponseCampaignsItemBuilder::status)
    /// - [`recipients`](GetSmsCampaignsResponseCampaignsItemBuilder::recipients)
    /// - [`statistics`](GetSmsCampaignsResponseCampaignsItemBuilder::statistics)
    pub fn build(self) -> Result<GetSmsCampaignsResponseCampaignsItem, BuildError> {
        Ok(GetSmsCampaignsResponseCampaignsItem {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organisation_prefix: self.organisation_prefix,
            scheduled_at: self.scheduled_at,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            sent_date: self.sent_date,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            unsubscribe_instruction: self.unsubscribe_instruction,
            recipients: self
                .recipients
                .ok_or_else(|| BuildError::missing_field("recipients"))?,
            statistics: self
                .statistics
                .ok_or_else(|| BuildError::missing_field("statistics"))?,
        })
    }
}
