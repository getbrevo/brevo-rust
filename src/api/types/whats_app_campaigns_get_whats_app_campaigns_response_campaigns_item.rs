pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetWhatsAppCampaignsResponseCampaignsItem {
    /// Name of the WhatsApp Campaign
    #[serde(rename = "campaignName")]
    #[serde(default)]
    pub campaign_name: String,
    /// Status of the Whatsapp Campaign
    #[serde(rename = "campaignStatus")]
    pub campaign_status: GetWhatsAppCampaignsResponseCampaignsItemCampaignStatus,
    /// Creation UTC date-time of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Error Reason associated with the WhatsApp campaign sending
    #[serde(rename = "errorReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    /// ID of the WhatsApp Campaign
    #[serde(default)]
    pub id: i64,
    /// Count of invalidated contacts
    #[serde(rename = "invalidatedContacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidated_contacts: Option<i64>,
    /// UTC date-time of last modification of the WhatsApp campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Read percentage of the the whatsapp campaign created
    #[serde(rename = "readPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub read_percentage: Option<f64>,
    /// UTC date-time on which WhatsApp campaign is scheduled. Should be in YYYY-MM-DDTHH:mm:ss.SSSZ format
    #[serde(rename = "scheduledAt")]
    #[serde(default)]
    pub scheduled_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<WhatsappCampStats>,
    /// Id of the WhatsApp template
    #[serde(rename = "templateId")]
    #[serde(default)]
    pub template_id: String,
}

impl GetWhatsAppCampaignsResponseCampaignsItem {
    pub fn builder() -> GetWhatsAppCampaignsResponseCampaignsItemBuilder {
        <GetWhatsAppCampaignsResponseCampaignsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppCampaignsResponseCampaignsItemBuilder {
    campaign_name: Option<String>,
    campaign_status: Option<GetWhatsAppCampaignsResponseCampaignsItemCampaignStatus>,
    created_at: Option<String>,
    error_reason: Option<String>,
    id: Option<i64>,
    invalidated_contacts: Option<i64>,
    modified_at: Option<String>,
    read_percentage: Option<f64>,
    scheduled_at: Option<String>,
    stats: Option<WhatsappCampStats>,
    template_id: Option<String>,
}

impl GetWhatsAppCampaignsResponseCampaignsItemBuilder {
    pub fn campaign_name(mut self, value: impl Into<String>) -> Self {
        self.campaign_name = Some(value.into());
        self
    }

    pub fn campaign_status(
        mut self,
        value: GetWhatsAppCampaignsResponseCampaignsItemCampaignStatus,
    ) -> Self {
        self.campaign_status = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn error_reason(mut self, value: impl Into<String>) -> Self {
        self.error_reason = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn invalidated_contacts(mut self, value: i64) -> Self {
        self.invalidated_contacts = Some(value);
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
        self
    }

    pub fn read_percentage(mut self, value: f64) -> Self {
        self.read_percentage = Some(value);
        self
    }

    pub fn scheduled_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_at = Some(value.into());
        self
    }

    pub fn stats(mut self, value: WhatsappCampStats) -> Self {
        self.stats = Some(value);
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppCampaignsResponseCampaignsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_name`](GetWhatsAppCampaignsResponseCampaignsItemBuilder::campaign_name)
    /// - [`campaign_status`](GetWhatsAppCampaignsResponseCampaignsItemBuilder::campaign_status)
    /// - [`created_at`](GetWhatsAppCampaignsResponseCampaignsItemBuilder::created_at)
    /// - [`id`](GetWhatsAppCampaignsResponseCampaignsItemBuilder::id)
    /// - [`modified_at`](GetWhatsAppCampaignsResponseCampaignsItemBuilder::modified_at)
    /// - [`scheduled_at`](GetWhatsAppCampaignsResponseCampaignsItemBuilder::scheduled_at)
    /// - [`template_id`](GetWhatsAppCampaignsResponseCampaignsItemBuilder::template_id)
    pub fn build(self) -> Result<GetWhatsAppCampaignsResponseCampaignsItem, BuildError> {
        Ok(GetWhatsAppCampaignsResponseCampaignsItem {
            campaign_name: self
                .campaign_name
                .ok_or_else(|| BuildError::missing_field("campaign_name"))?,
            campaign_status: self
                .campaign_status
                .ok_or_else(|| BuildError::missing_field("campaign_status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            error_reason: self.error_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invalidated_contacts: self.invalidated_contacts,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            read_percentage: self.read_percentage,
            scheduled_at: self
                .scheduled_at
                .ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
            stats: self.stats,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
        })
    }
}
