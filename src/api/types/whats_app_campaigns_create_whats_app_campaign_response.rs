pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWhatsAppCampaignResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateWhatsAppCampaignResponse {
    pub fn builder() -> CreateWhatsAppCampaignResponseBuilder {
        <CreateWhatsAppCampaignResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWhatsAppCampaignResponseBuilder {
    id: Option<i64>,
}

impl CreateWhatsAppCampaignResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWhatsAppCampaignResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateWhatsAppCampaignResponseBuilder::id)
    pub fn build(self) -> Result<CreateWhatsAppCampaignResponse, BuildError> {
        Ok(CreateWhatsAppCampaignResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
