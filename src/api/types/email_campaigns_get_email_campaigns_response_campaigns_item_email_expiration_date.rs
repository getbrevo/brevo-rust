pub use crate::prelude::*;

/// Expiration date configuration for the email campaign, if set. Contains the duration and unit of the email expiry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailCampaignsResponseCampaignsItemEmailExpirationDate {
    /// Duration of the email expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Unit of the duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<GetEmailCampaignsResponseCampaignsItemEmailExpirationDateUnit>,
}

impl GetEmailCampaignsResponseCampaignsItemEmailExpirationDate {
    pub fn builder() -> GetEmailCampaignsResponseCampaignsItemEmailExpirationDateBuilder {
        <GetEmailCampaignsResponseCampaignsItemEmailExpirationDateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignsResponseCampaignsItemEmailExpirationDateBuilder {
    duration: Option<i64>,
    unit: Option<GetEmailCampaignsResponseCampaignsItemEmailExpirationDateUnit>,
}

impl GetEmailCampaignsResponseCampaignsItemEmailExpirationDateBuilder {
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn unit(
        mut self,
        value: GetEmailCampaignsResponseCampaignsItemEmailExpirationDateUnit,
    ) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailCampaignsResponseCampaignsItemEmailExpirationDate`].
    pub fn build(
        self,
    ) -> Result<GetEmailCampaignsResponseCampaignsItemEmailExpirationDate, BuildError> {
        Ok(GetEmailCampaignsResponseCampaignsItemEmailExpirationDate {
            duration: self.duration,
            unit: self.unit,
        })
    }
}
