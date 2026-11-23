pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RedeemVoucherRequestOrderBilling {
    /// Address of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// City of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country code of the user
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Payment method opted by the user
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Phone number of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<i64>,
    /// Postal Code of user's location
    #[serde(rename = "postCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<i64>,
    /// Region where user resides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl RedeemVoucherRequestOrderBilling {
    pub fn builder() -> RedeemVoucherRequestOrderBillingBuilder {
        <RedeemVoucherRequestOrderBillingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemVoucherRequestOrderBillingBuilder {
    address: Option<String>,
    city: Option<String>,
    country_code: Option<String>,
    payment_method: Option<String>,
    phone: Option<i64>,
    post_code: Option<i64>,
    region: Option<String>,
}

impl RedeemVoucherRequestOrderBillingBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
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

    pub fn phone(mut self, value: i64) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn post_code(mut self, value: i64) -> Self {
        self.post_code = Some(value);
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RedeemVoucherRequestOrderBilling`].
    pub fn build(self) -> Result<RedeemVoucherRequestOrderBilling, BuildError> {
        Ok(RedeemVoucherRequestOrderBilling {
            address: self.address,
            city: self.city,
            country_code: self.country_code,
            payment_method: self.payment_method,
            phone: self.phone,
            post_code: self.post_code,
            region: self.region,
        })
    }
}
