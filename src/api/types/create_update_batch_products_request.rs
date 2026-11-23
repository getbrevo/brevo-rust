pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateUpdateBatchProductsRequest {
    /// array of products objects
    #[serde(default)]
    pub products: Vec<CreateUpdateBatchProductsRequestProductsItem>,
    /// Facilitate to update the existing products in the same request (updateEnabled = true)
    #[serde(rename = "updateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_enabled: Option<bool>,
}

impl CreateUpdateBatchProductsRequest {
    pub fn builder() -> CreateUpdateBatchProductsRequestBuilder {
        <CreateUpdateBatchProductsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateBatchProductsRequestBuilder {
    products: Option<Vec<CreateUpdateBatchProductsRequestProductsItem>>,
    update_enabled: Option<bool>,
}

impl CreateUpdateBatchProductsRequestBuilder {
    pub fn products(mut self, value: Vec<CreateUpdateBatchProductsRequestProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    pub fn update_enabled(mut self, value: bool) -> Self {
        self.update_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateBatchProductsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`products`](CreateUpdateBatchProductsRequestBuilder::products)
    pub fn build(self) -> Result<CreateUpdateBatchProductsRequest, BuildError> {
        Ok(CreateUpdateBatchProductsRequest {
            products: self
                .products
                .ok_or_else(|| BuildError::missing_field("products"))?,
            update_enabled: self.update_enabled,
        })
    }
}
