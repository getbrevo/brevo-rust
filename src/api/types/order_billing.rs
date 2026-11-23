pub use crate::prelude::*;

/// Billing details of an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrderBilling {
    /// Full billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Exact city of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Billing country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Billing country 2-letter ISO code.
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// How the visitor will pay for the item(s), e.g. paypal, check, etc.
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Billing phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Billing postcode.
    #[serde(rename = "postCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// Exact region (state/province) for billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl OrderBilling {
    pub fn builder() -> OrderBillingBuilder {
        <OrderBillingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderBillingBuilder {
    address: Option<String>,
    city: Option<String>,
    country: Option<String>,
    country_code: Option<String>,
    payment_method: Option<String>,
    phone: Option<String>,
    post_code: Option<String>,
    region: Option<String>,
}

impl OrderBillingBuilder {
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

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
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

    /// Consumes the builder and constructs a [`OrderBilling`].
    pub fn build(self) -> Result<OrderBilling, BuildError> {
        Ok(OrderBilling {
            address: self.address,
            city: self.city,
            country: self.country,
            country_code: self.country_code,
            payment_method: self.payment_method,
            phone: self.phone,
            post_code: self.post_code,
            region: self.region,
        })
    }
}
