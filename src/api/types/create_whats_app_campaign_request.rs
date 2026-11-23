pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWhatsAppCampaignRequest {
    /// Name of the WhatsApp campaign creation
    #[serde(default)]
    pub name: String,
    /// Segment ids and List ids to include/exclude from campaign
    #[serde(default)]
    pub recipients: CreateWhatsAppCampaignRequestRecipients,
    /// Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.For example: **2017-06-01T12:30:00+02:00**
    #[serde(rename = "scheduledAt")]
    #[serde(default)]
    pub scheduled_at: String,
    /// Id of the WhatsApp template in **approved** state
    #[serde(rename = "templateId")]
    #[serde(default)]
    pub template_id: i64,
}

impl CreateWhatsAppCampaignRequest {
    pub fn builder() -> CreateWhatsAppCampaignRequestBuilder {
        <CreateWhatsAppCampaignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWhatsAppCampaignRequestBuilder {
    name: Option<String>,
    recipients: Option<CreateWhatsAppCampaignRequestRecipients>,
    scheduled_at: Option<String>,
    template_id: Option<i64>,
}

impl CreateWhatsAppCampaignRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: CreateWhatsAppCampaignRequestRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn scheduled_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_at = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWhatsAppCampaignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateWhatsAppCampaignRequestBuilder::name)
    /// - [`recipients`](CreateWhatsAppCampaignRequestBuilder::recipients)
    /// - [`scheduled_at`](CreateWhatsAppCampaignRequestBuilder::scheduled_at)
    /// - [`template_id`](CreateWhatsAppCampaignRequestBuilder::template_id)
    pub fn build(self) -> Result<CreateWhatsAppCampaignRequest, BuildError> {
        Ok(CreateWhatsAppCampaignRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            recipients: self
                .recipients
                .ok_or_else(|| BuildError::missing_field("recipients"))?,
            scheduled_at: self
                .scheduled_at
                .ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
        })
    }
}
