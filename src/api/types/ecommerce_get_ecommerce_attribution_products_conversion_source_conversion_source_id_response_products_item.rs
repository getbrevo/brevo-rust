pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ordersCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub revenue: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem {
    pub fn builder(
    ) -> GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItemBuilder
    {
        <GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItemBuilder
{
    id: Option<String>,
    image_url: Option<String>,
    name: Option<String>,
    orders_count: Option<i64>,
    price: Option<f64>,
    revenue: Option<f64>,
    sku: Option<String>,
    url: Option<String>,
}

impl GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn orders_count(mut self, value: i64) -> Self {
        self.orders_count = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn revenue(mut self, value: f64) -> Self {
        self.revenue = Some(value);
        self
    }

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItemBuilder::id)
    pub fn build(
        self,
    ) -> Result<
        GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem,
        BuildError,
    > {
        Ok(
            GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponseProductsItem {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                image_url: self.image_url,
                name: self.name,
                orders_count: self.orders_count,
                price: self.price,
                revenue: self.revenue,
                sku: self.sku,
                url: self.url,
            },
        )
    }
}
