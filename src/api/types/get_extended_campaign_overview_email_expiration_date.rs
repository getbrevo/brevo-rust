pub use crate::prelude::*;

/// Expiration date configuration for the email campaign, if set. Contains the duration and unit of the email expiry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetExtendedCampaignOverviewEmailExpirationDate {
    /// Duration of the email expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Unit of the duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<GetExtendedCampaignOverviewEmailExpirationDateUnit>,
}

impl GetExtendedCampaignOverviewEmailExpirationDate {
    pub fn builder() -> GetExtendedCampaignOverviewEmailExpirationDateBuilder {
        <GetExtendedCampaignOverviewEmailExpirationDateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetExtendedCampaignOverviewEmailExpirationDateBuilder {
    duration: Option<i64>,
    unit: Option<GetExtendedCampaignOverviewEmailExpirationDateUnit>,
}

impl GetExtendedCampaignOverviewEmailExpirationDateBuilder {
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn unit(mut self, value: GetExtendedCampaignOverviewEmailExpirationDateUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetExtendedCampaignOverviewEmailExpirationDate`].
    pub fn build(self) -> Result<GetExtendedCampaignOverviewEmailExpirationDate, BuildError> {
        Ok(GetExtendedCampaignOverviewEmailExpirationDate {
            duration: self.duration,
            unit: self.unit,
        })
    }
}
