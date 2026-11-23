pub use crate::prelude::*;

/// Expiration date configuration for the email campaign, if set. Contains the duration and unit of the email expiry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailCampaignResponseEmailExpirationDate {
    /// Duration of the email expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Unit of the duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<GetEmailCampaignResponseEmailExpirationDateUnit>,
}

impl GetEmailCampaignResponseEmailExpirationDate {
    pub fn builder() -> GetEmailCampaignResponseEmailExpirationDateBuilder {
        <GetEmailCampaignResponseEmailExpirationDateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignResponseEmailExpirationDateBuilder {
    duration: Option<i64>,
    unit: Option<GetEmailCampaignResponseEmailExpirationDateUnit>,
}

impl GetEmailCampaignResponseEmailExpirationDateBuilder {
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn unit(mut self, value: GetEmailCampaignResponseEmailExpirationDateUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailCampaignResponseEmailExpirationDate`].
    pub fn build(self) -> Result<GetEmailCampaignResponseEmailExpirationDate, BuildError> {
        Ok(GetEmailCampaignResponseEmailExpirationDate {
            duration: self.duration,
            unit: self.unit,
        })
    }
}
