pub use crate::prelude::*;

/// Billing address of master account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateMasterAccountResponseBillingInfoAddress {
    /// Country code
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Locality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Postal code
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State code
    #[serde(rename = "stateCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// Street address
    #[serde(rename = "streetAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
}

impl GetCorporateMasterAccountResponseBillingInfoAddress {
    pub fn builder() -> GetCorporateMasterAccountResponseBillingInfoAddressBuilder {
        <GetCorporateMasterAccountResponseBillingInfoAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateMasterAccountResponseBillingInfoAddressBuilder {
    country_code: Option<String>,
    locality: Option<String>,
    postal_code: Option<String>,
    state_code: Option<String>,
    street_address: Option<String>,
}

impl GetCorporateMasterAccountResponseBillingInfoAddressBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn locality(mut self, value: impl Into<String>) -> Self {
        self.locality = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state_code(mut self, value: impl Into<String>) -> Self {
        self.state_code = Some(value.into());
        self
    }

    pub fn street_address(mut self, value: impl Into<String>) -> Self {
        self.street_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateMasterAccountResponseBillingInfoAddress`].
    pub fn build(self) -> Result<GetCorporateMasterAccountResponseBillingInfoAddress, BuildError> {
        Ok(GetCorporateMasterAccountResponseBillingInfoAddress {
            country_code: self.country_code,
            locality: self.locality,
            postal_code: self.postal_code,
            state_code: self.state_code,
            street_address: self.street_address,
        })
    }
}
