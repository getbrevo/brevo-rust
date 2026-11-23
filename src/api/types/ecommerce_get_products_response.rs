pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetProductsResponse {
    /// Number of products
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub products: Vec<GetProductDetails>,
}

impl GetProductsResponse {
    pub fn builder() -> GetProductsResponseBuilder {
        <GetProductsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProductsResponseBuilder {
    count: Option<i64>,
    products: Option<Vec<GetProductDetails>>,
}

impl GetProductsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<GetProductDetails>) -> Self {
        self.products = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProductsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](GetProductsResponseBuilder::count)
    /// - [`products`](GetProductsResponseBuilder::products)
    pub fn build(self) -> Result<GetProductsResponse, BuildError> {
        Ok(GetProductsResponse {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            products: self
                .products
                .ok_or_else(|| BuildError::missing_field("products"))?,
        })
    }
}
