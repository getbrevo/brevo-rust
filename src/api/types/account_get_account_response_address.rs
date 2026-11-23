pub use crate::prelude::*;

/// Address informations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountResponseAddress {
    /// City information
    #[serde(default)]
    pub city: String,
    /// Country information
    #[serde(default)]
    pub country: String,
    /// Street information
    #[serde(default)]
    pub street: String,
    /// Zip Code information
    #[serde(rename = "zipCode")]
    #[serde(default)]
    pub zip_code: String,
}

impl GetAccountResponseAddress {
    pub fn builder() -> GetAccountResponseAddressBuilder {
        <GetAccountResponseAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    street: Option<String>,
    zip_code: Option<String>,
}

impl GetAccountResponseAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponseAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](GetAccountResponseAddressBuilder::city)
    /// - [`country`](GetAccountResponseAddressBuilder::country)
    /// - [`street`](GetAccountResponseAddressBuilder::street)
    /// - [`zip_code`](GetAccountResponseAddressBuilder::zip_code)
    pub fn build(self) -> Result<GetAccountResponseAddress, BuildError> {
        Ok(GetAccountResponseAddress {
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            street: self
                .street
                .ok_or_else(|| BuildError::missing_field("street"))?,
            zip_code: self
                .zip_code
                .ok_or_else(|| BuildError::missing_field("zip_code"))?,
        })
    }
}
