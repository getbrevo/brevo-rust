pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RedeemVoucherRequestOrder {
    /// Total amount of the order
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Billing information for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<RedeemVoucherRequestOrderBilling>,
    /// Unique identifier for the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// List of coupon codes applied to the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<Vec<String>>,
    /// Timestamp when the order was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Email address associated with the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Unique identifier for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional identifiers for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<RedeemVoucherRequestOrderIdentifiers>,
    /// List of products in the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<RedeemVoucherRequestOrderProductsItem>>,
    /// Current status of the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Identifier for the store where the order was placed
    #[serde(rename = "storeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// Timestamp when the order was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl RedeemVoucherRequestOrder {
    pub fn builder() -> RedeemVoucherRequestOrderBuilder {
        <RedeemVoucherRequestOrderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemVoucherRequestOrderBuilder {
    amount: Option<f64>,
    billing: Option<RedeemVoucherRequestOrderBilling>,
    contact_id: Option<i64>,
    coupons: Option<Vec<String>>,
    created_at: Option<DateTime<FixedOffset>>,
    email: Option<String>,
    id: Option<String>,
    identifiers: Option<RedeemVoucherRequestOrderIdentifiers>,
    products: Option<Vec<RedeemVoucherRequestOrderProductsItem>>,
    status: Option<String>,
    store_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl RedeemVoucherRequestOrderBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn billing(mut self, value: RedeemVoucherRequestOrderBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn coupons(mut self, value: Vec<String>) -> Self {
        self.coupons = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifiers(mut self, value: RedeemVoucherRequestOrderIdentifiers) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<RedeemVoucherRequestOrderProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RedeemVoucherRequestOrder`].
    pub fn build(self) -> Result<RedeemVoucherRequestOrder, BuildError> {
        Ok(RedeemVoucherRequestOrder {
            amount: self.amount,
            billing: self.billing,
            contact_id: self.contact_id,
            coupons: self.coupons,
            created_at: self.created_at,
            email: self.email,
            id: self.id,
            identifiers: self.identifiers,
            products: self.products,
            status: self.status,
            store_id: self.store_id,
            updated_at: self.updated_at,
        })
    }
}
