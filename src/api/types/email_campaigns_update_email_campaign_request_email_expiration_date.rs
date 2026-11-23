pub use crate::prelude::*;

/// To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEmailCampaignRequestEmailExpirationDate {
    /// Duration of the email expiry. maximum duration can be 3600 days or 480 weeks or 120 months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// unit of the duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<UpdateEmailCampaignRequestEmailExpirationDateUnit>,
}

impl UpdateEmailCampaignRequestEmailExpirationDate {
    pub fn builder() -> UpdateEmailCampaignRequestEmailExpirationDateBuilder {
        <UpdateEmailCampaignRequestEmailExpirationDateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailCampaignRequestEmailExpirationDateBuilder {
    duration: Option<i64>,
    unit: Option<UpdateEmailCampaignRequestEmailExpirationDateUnit>,
}

impl UpdateEmailCampaignRequestEmailExpirationDateBuilder {
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn unit(mut self, value: UpdateEmailCampaignRequestEmailExpirationDateUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailCampaignRequestEmailExpirationDate`].
    pub fn build(self) -> Result<UpdateEmailCampaignRequestEmailExpirationDate, BuildError> {
        Ok(UpdateEmailCampaignRequestEmailExpirationDate {
            duration: self.duration,
            unit: self.unit,
        })
    }
}
