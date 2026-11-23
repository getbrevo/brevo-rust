pub use crate::prelude::*;

/// Billing name of master account holder
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateMasterAccountResponseBillingInfoName {
    /// Last name for billing
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// First name for billing
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
}

impl GetCorporateMasterAccountResponseBillingInfoName {
    pub fn builder() -> GetCorporateMasterAccountResponseBillingInfoNameBuilder {
        <GetCorporateMasterAccountResponseBillingInfoNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateMasterAccountResponseBillingInfoNameBuilder {
    family_name: Option<String>,
    given_name: Option<String>,
}

impl GetCorporateMasterAccountResponseBillingInfoNameBuilder {
    pub fn family_name(mut self, value: impl Into<String>) -> Self {
        self.family_name = Some(value.into());
        self
    }

    pub fn given_name(mut self, value: impl Into<String>) -> Self {
        self.given_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateMasterAccountResponseBillingInfoName`].
    pub fn build(self) -> Result<GetCorporateMasterAccountResponseBillingInfoName, BuildError> {
        Ok(GetCorporateMasterAccountResponseBillingInfoName {
            family_name: self.family_name,
            given_name: self.given_name,
        })
    }
}
