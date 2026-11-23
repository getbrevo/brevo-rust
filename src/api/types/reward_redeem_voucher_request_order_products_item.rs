pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RedeemVoucherRequestOrderProductsItem {
    /// List of categories the product belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    /// Price of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    /// Unique identifier for the product
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Quantity of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub quantity: Option<f64>,
    /// Identifier for the product variant
    #[serde(rename = "variantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl RedeemVoucherRequestOrderProductsItem {
    pub fn builder() -> RedeemVoucherRequestOrderProductsItemBuilder {
        <RedeemVoucherRequestOrderProductsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemVoucherRequestOrderProductsItemBuilder {
    category: Option<Vec<String>>,
    price: Option<f64>,
    product_id: Option<String>,
    quantity: Option<f64>,
    variant_id: Option<String>,
}

impl RedeemVoucherRequestOrderProductsItemBuilder {
    pub fn category(mut self, value: Vec<String>) -> Self {
        self.category = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RedeemVoucherRequestOrderProductsItem`].
    pub fn build(self) -> Result<RedeemVoucherRequestOrderProductsItem, BuildError> {
        Ok(RedeemVoucherRequestOrderProductsItem {
            category: self.category,
            price: self.price,
            product_id: self.product_id,
            quantity: self.quantity,
            variant_id: self.variant_id,
        })
    }
}
