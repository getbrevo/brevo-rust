pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetWhatsAppCampaignResponse {
    /// Name of the WhatsApp Campaign
    #[serde(rename = "campaignName")]
    #[serde(default)]
    pub campaign_name: String,
    /// Status of the WhatsApp Campaign
    #[serde(rename = "campaignStatus")]
    pub campaign_status: GetWhatsAppCampaignResponseCampaignStatus,
    /// Creation UTC date-time of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// ID of the WhatsApp Campaign
    #[serde(default)]
    pub id: i64,
    /// UTC date-time of last modification of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// UTC date-time on which WhatsApp campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Recipients of the WhatsApp Campaign
    #[serde(default)]
    pub recipients: GetWhatsAppCampaignResponseRecipients,
    /// Sender of the WhatsApp Campaign
    #[serde(rename = "senderNumber")]
    #[serde(default)]
    pub sender_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<WhatsappCampStats>,
    #[serde(default)]
    pub template: GetWhatsAppCampaignResponseTemplate,
}

impl GetWhatsAppCampaignResponse {
    pub fn builder() -> GetWhatsAppCampaignResponseBuilder {
        <GetWhatsAppCampaignResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppCampaignResponseBuilder {
    campaign_name: Option<String>,
    campaign_status: Option<GetWhatsAppCampaignResponseCampaignStatus>,
    created_at: Option<String>,
    id: Option<i64>,
    modified_at: Option<String>,
    scheduled_at: Option<String>,
    recipients: Option<GetWhatsAppCampaignResponseRecipients>,
    sender_number: Option<String>,
    stats: Option<WhatsappCampStats>,
    template: Option<GetWhatsAppCampaignResponseTemplate>,
}

impl GetWhatsAppCampaignResponseBuilder {
    pub fn campaign_name(mut self, value: impl Into<String>) -> Self {
        self.campaign_name = Some(value.into());
        self
    }

    pub fn campaign_status(mut self, value: GetWhatsAppCampaignResponseCampaignStatus) -> Self {
        self.campaign_status = Some(value);
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

    pub fn scheduled_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_at = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: GetWhatsAppCampaignResponseRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn sender_number(mut self, value: impl Into<String>) -> Self {
        self.sender_number = Some(value.into());
        self
    }

    pub fn stats(mut self, value: WhatsappCampStats) -> Self {
        self.stats = Some(value);
        self
    }

    pub fn template(mut self, value: GetWhatsAppCampaignResponseTemplate) -> Self {
        self.template = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppCampaignResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_name`](GetWhatsAppCampaignResponseBuilder::campaign_name)
    /// - [`campaign_status`](GetWhatsAppCampaignResponseBuilder::campaign_status)
    /// - [`created_at`](GetWhatsAppCampaignResponseBuilder::created_at)
    /// - [`id`](GetWhatsAppCampaignResponseBuilder::id)
    /// - [`modified_at`](GetWhatsAppCampaignResponseBuilder::modified_at)
    /// - [`recipients`](GetWhatsAppCampaignResponseBuilder::recipients)
    /// - [`sender_number`](GetWhatsAppCampaignResponseBuilder::sender_number)
    /// - [`template`](GetWhatsAppCampaignResponseBuilder::template)
    pub fn build(self) -> Result<GetWhatsAppCampaignResponse, BuildError> {
        Ok(GetWhatsAppCampaignResponse {
            campaign_name: self
                .campaign_name
                .ok_or_else(|| BuildError::missing_field("campaign_name"))?,
            campaign_status: self
                .campaign_status
                .ok_or_else(|| BuildError::missing_field("campaign_status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            scheduled_at: self.scheduled_at,
            recipients: self
                .recipients
                .ok_or_else(|| BuildError::missing_field("recipients"))?,
            sender_number: self
                .sender_number
                .ok_or_else(|| BuildError::missing_field("sender_number"))?,
            stats: self.stats,
            template: self
                .template
                .ok_or_else(|| BuildError::missing_field("template"))?,
        })
    }
}
