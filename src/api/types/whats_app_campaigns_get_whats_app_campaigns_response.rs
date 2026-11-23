pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetWhatsAppCampaignsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<Vec<GetWhatsAppCampaignsResponseCampaignsItem>>,
    /// Number of WhatsApp campaigns retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl GetWhatsAppCampaignsResponse {
    pub fn builder() -> GetWhatsAppCampaignsResponseBuilder {
        <GetWhatsAppCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppCampaignsResponseBuilder {
    campaigns: Option<Vec<GetWhatsAppCampaignsResponseCampaignsItem>>,
    count: Option<i64>,
}

impl GetWhatsAppCampaignsResponseBuilder {
    pub fn campaigns(mut self, value: Vec<GetWhatsAppCampaignsResponseCampaignsItem>) -> Self {
        self.campaigns = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppCampaignsResponse`].
    pub fn build(self) -> Result<GetWhatsAppCampaignsResponse, BuildError> {
        Ok(GetWhatsAppCampaignsResponse {
            campaigns: self.campaigns,
            count: self.count,
        })
    }
}
