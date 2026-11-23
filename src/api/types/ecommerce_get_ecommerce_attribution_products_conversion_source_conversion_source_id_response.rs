pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse {
    /// List of attributed products
    #[serde(default)]
    pub products:
        Vec<GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem>,
}

impl GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse {
    pub fn builder(
    ) -> GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseBuilder {
        <GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseBuilder {
    products: Option<
        Vec<GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem>,
    >,
}

impl GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseBuilder {
    pub fn products(
        mut self,
        value: Vec<
            GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem,
        >,
    ) -> Self {
        self.products = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`products`](GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseBuilder::products)
    pub fn build(
        self,
    ) -> Result<GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse, BuildError>
    {
        Ok(
            GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse {
                products: self
                    .products
                    .ok_or_else(|| BuildError::missing_field("products"))?,
            },
        )
    }
}
