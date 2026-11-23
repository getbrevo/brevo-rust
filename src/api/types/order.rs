pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Order {
    /// Total amount of the order, including all shipping expenses, tax and the price of items.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Billing details of an order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<OrderBilling>,
    /// Coupons applied to the order. Stored case insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<Vec<String>>,
    /// UTC date-time when the order was created, in ISO 8601 format ending in `Z` (for example `2021-07-29T20:59:23Z`). Fractional seconds are accepted (`2021-07-29T20:59:23.383Z`). Timezone offsets such as `+01:00` are not supported: convert the date to UTC first.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Unique ID of the order.
    #[serde(default)]
    pub id: String,
    /// Identifies the contact associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<OrderIdentifiers>,
    #[serde(default)]
    pub products: Vec<OrderProductsItem>,
    /// Shipping details of an order, when the recipient or ship-to address differs from billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderShipping>,
    /// State of the order.
    #[serde(default)]
    pub status: String,
    /// ID of store where the order is placed
    #[serde(rename = "storeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// UTC date-time when the order status was last changed, in ISO 8601 format ending in `Z` (for example `2021-07-30T10:59:23Z`). Fractional seconds are accepted (`2021-07-30T10:59:23.383Z`). Timezone offsets such as `+01:00` are not supported: convert the date to UTC first.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    /// Additional order details such as a custom message, customer type or source. Values can be strings, numbers or booleans. Strings in RFC 3339 format (for example `2021-07-29T20:59:23Z`) are stored as dates.
    #[serde(rename = "metaInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_info: Option<HashMap<String, OrderMetaInfoValue>>,
}

impl Order {
    pub fn builder() -> OrderBuilder {
        <OrderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderBuilder {
    amount: Option<f64>,
    billing: Option<OrderBilling>,
    coupons: Option<Vec<String>>,
    created_at: Option<String>,
    id: Option<String>,
    identifiers: Option<OrderIdentifiers>,
    products: Option<Vec<OrderProductsItem>>,
    shipping: Option<OrderShipping>,
    status: Option<String>,
    store_id: Option<String>,
    updated_at: Option<String>,
    meta_info: Option<HashMap<String, OrderMetaInfoValue>>,
}

impl OrderBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn billing(mut self, value: OrderBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn coupons(mut self, value: Vec<String>) -> Self {
        self.coupons = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifiers(mut self, value: OrderIdentifiers) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<OrderProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    pub fn shipping(mut self, value: OrderShipping) -> Self {
        self.shipping = Some(value);
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn meta_info(mut self, value: HashMap<String, OrderMetaInfoValue>) -> Self {
        self.meta_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Order`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](OrderBuilder::amount)
    /// - [`created_at`](OrderBuilder::created_at)
    /// - [`id`](OrderBuilder::id)
    /// - [`products`](OrderBuilder::products)
    /// - [`status`](OrderBuilder::status)
    /// - [`updated_at`](OrderBuilder::updated_at)
    pub fn build(self) -> Result<Order, BuildError> {
        Ok(Order {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            billing: self.billing,
            coupons: self.coupons,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identifiers: self.identifiers,
            products: self
                .products
                .ok_or_else(|| BuildError::missing_field("products"))?,
            shipping: self.shipping,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            store_id: self.store_id,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            meta_info: self.meta_info,
        })
    }
}
