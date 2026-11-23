pub use crate::prelude::*;

/// Details for the Products in an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrderProductsItem {
    /// The price of a unit of product
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub price: f64,
    /// ID of the product.
    #[serde(rename = "productId")]
    #[serde(default)]
    pub product_id: String,
    /// Stock Keeping Unit identifier for the product line item. Stored as sent: it is not matched against the product catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Product ID of the red color shirts.
    #[serde(rename = "variantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    /// **Required if quantityFloat is empty.**
    ///
    /// Number of product units added to the cart (whole numbers only, e.g., 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// **Required if quantity is empty.**
    ///
    /// Number of product units added to the cart(supports decimals, e.g., 20.52)
    #[serde(rename = "quantityFloat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub quantity_float: Option<f64>,
}

impl OrderProductsItem {
    pub fn builder() -> OrderProductsItemBuilder {
        <OrderProductsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderProductsItemBuilder {
    price: Option<f64>,
    product_id: Option<String>,
    sku: Option<String>,
    variant_id: Option<String>,
    quantity: Option<i64>,
    quantity_float: Option<f64>,
}

impl OrderProductsItemBuilder {
    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn quantity_float(mut self, value: f64) -> Self {
        self.quantity_float = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrderProductsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price`](OrderProductsItemBuilder::price)
    /// - [`product_id`](OrderProductsItemBuilder::product_id)
    pub fn build(self) -> Result<OrderProductsItem, BuildError> {
        Ok(OrderProductsItem {
            price: self
                .price
                .ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
            sku: self.sku,
            variant_id: self.variant_id,
            quantity: self.quantity,
            quantity_float: self.quantity_float,
        })
    }
}
