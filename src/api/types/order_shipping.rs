pub use crate::prelude::*;

/// Shipping details of an order, when the recipient or ship-to address differs from billing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrderShipping {
    /// Full shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Exact city of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Shipping country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Shipping country 2-letter ISO code.
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Name of the shipping recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Phone number to contact for shipping questions. If invalid, it is dropped and the order is still accepted. Unlike `billing.phone`, it is never used to identify the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Shipping postcode.
    #[serde(rename = "postCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// Exact region (state/province) for shipping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl OrderShipping {
    pub fn builder() -> OrderShippingBuilder {
        <OrderShippingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderShippingBuilder {
    address: Option<String>,
    city: Option<String>,
    country: Option<String>,
    country_code: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    post_code: Option<String>,
    region: Option<String>,
}

impl OrderShippingBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn post_code(mut self, value: impl Into<String>) -> Self {
        self.post_code = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrderShipping`].
    pub fn build(self) -> Result<OrderShipping, BuildError> {
        Ok(OrderShipping {
            address: self.address,
            city: self.city,
            country: self.country,
            country_code: self.country_code,
            name: self.name,
            phone: self.phone,
            post_code: self.post_code,
            region: self.region,
        })
    }
}
