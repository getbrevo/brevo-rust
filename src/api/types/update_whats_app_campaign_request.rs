pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWhatsAppCampaignRequest {
    /// Name of the campaign
    #[serde(rename = "campaignName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_name: Option<String>,
    /// Status of the campaign
    #[serde(rename = "campaignStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_status: Option<UpdateWhatsAppCampaignRequestCampaignStatus>,
    /// Segment ids and List ids to include/exclude from campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<UpdateWhatsAppCampaignRequestRecipients>,
    /// Reschedule the sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of campaign. **Prefer to pass your timezone in date-time format for accurate result.For example: **2017-06-01T12:30:00+02:00** Use this field to update the scheduledAt of any existing draft or scheduled WhatsApp campaign.
    #[serde(rename = "rescheduleFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reschedule_for: Option<String>,
}

impl UpdateWhatsAppCampaignRequest {
    pub fn builder() -> UpdateWhatsAppCampaignRequestBuilder {
        <UpdateWhatsAppCampaignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWhatsAppCampaignRequestBuilder {
    campaign_name: Option<String>,
    campaign_status: Option<UpdateWhatsAppCampaignRequestCampaignStatus>,
    recipients: Option<UpdateWhatsAppCampaignRequestRecipients>,
    reschedule_for: Option<String>,
}

impl UpdateWhatsAppCampaignRequestBuilder {
    pub fn campaign_name(mut self, value: impl Into<String>) -> Self {
        self.campaign_name = Some(value.into());
        self
    }

    pub fn campaign_status(mut self, value: UpdateWhatsAppCampaignRequestCampaignStatus) -> Self {
        self.campaign_status = Some(value);
        self
    }

    pub fn recipients(mut self, value: UpdateWhatsAppCampaignRequestRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn reschedule_for(mut self, value: impl Into<String>) -> Self {
        self.reschedule_for = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWhatsAppCampaignRequest`].
    pub fn build(self) -> Result<UpdateWhatsAppCampaignRequest, BuildError> {
        Ok(UpdateWhatsAppCampaignRequest {
            campaign_name: self.campaign_name,
            campaign_status: self.campaign_status,
            recipients: self.recipients,
            reschedule_for: self.reschedule_for,
        })
    }
}
