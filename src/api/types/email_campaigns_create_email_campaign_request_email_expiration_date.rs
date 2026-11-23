pub use crate::prelude::*;

/// To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy. Learn more about setting an email expiration date. For reference , ``https://help.brevo.com/hc/en-us/articles/4413566705298-Create-an-email-campaign``
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEmailCampaignRequestEmailExpirationDate {
    /// Duration of the email expiry. maximum duration can be 3600 days or 480 weeks or 120 months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// unit of the duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<CreateEmailCampaignRequestEmailExpirationDateUnit>,
}

impl CreateEmailCampaignRequestEmailExpirationDate {
    pub fn builder() -> CreateEmailCampaignRequestEmailExpirationDateBuilder {
        <CreateEmailCampaignRequestEmailExpirationDateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEmailCampaignRequestEmailExpirationDateBuilder {
    duration: Option<i64>,
    unit: Option<CreateEmailCampaignRequestEmailExpirationDateUnit>,
}

impl CreateEmailCampaignRequestEmailExpirationDateBuilder {
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn unit(mut self, value: CreateEmailCampaignRequestEmailExpirationDateUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEmailCampaignRequestEmailExpirationDate`].
    pub fn build(self) -> Result<CreateEmailCampaignRequestEmailExpirationDate, BuildError> {
        Ok(CreateEmailCampaignRequestEmailExpirationDate {
            duration: self.duration,
            unit: self.unit,
        })
    }
}
