pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse {
    #[serde(rename = "averageBasket")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub average_basket: f64,
    #[serde(rename = "conversionSource")]
    pub conversion_source:
        GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseConversionSource,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub id: f64,
    #[serde(rename = "ordersCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub orders_count: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub revenue: f64,
    #[serde(rename = "newCustomersCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub new_customers_count: f64,
}

impl GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse {
    pub fn builder(
    ) -> GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder {
        <GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder {
    average_basket: Option<f64>,
    conversion_source: Option<
        GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseConversionSource,
    >,
    id: Option<f64>,
    orders_count: Option<f64>,
    revenue: Option<f64>,
    new_customers_count: Option<f64>,
}

impl GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder {
    pub fn average_basket(mut self, value: f64) -> Self {
        self.average_basket = Some(value);
        self
    }

    pub fn conversion_source(
        mut self,
        value: GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseConversionSource,
    ) -> Self {
        self.conversion_source = Some(value);
        self
    }

    pub fn id(mut self, value: f64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn orders_count(mut self, value: f64) -> Self {
        self.orders_count = Some(value);
        self
    }

    pub fn revenue(mut self, value: f64) -> Self {
        self.revenue = Some(value);
        self
    }

    pub fn new_customers_count(mut self, value: f64) -> Self {
        self.new_customers_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`average_basket`](GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder::average_basket)
    /// - [`conversion_source`](GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder::conversion_source)
    /// - [`id`](GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder::id)
    /// - [`orders_count`](GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder::orders_count)
    /// - [`revenue`](GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder::revenue)
    /// - [`new_customers_count`](GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponseBuilder::new_customers_count)
    pub fn build(
        self,
    ) -> Result<GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse, BuildError>
    {
        Ok(
            GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse {
                average_basket: self
                    .average_basket
                    .ok_or_else(|| BuildError::missing_field("average_basket"))?,
                conversion_source: self
                    .conversion_source
                    .ok_or_else(|| BuildError::missing_field("conversion_source"))?,
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                orders_count: self
                    .orders_count
                    .ok_or_else(|| BuildError::missing_field("orders_count"))?,
                revenue: self
                    .revenue
                    .ok_or_else(|| BuildError::missing_field("revenue"))?,
                new_customers_count: self
                    .new_customers_count
                    .ok_or_else(|| BuildError::missing_field("new_customers_count"))?,
            },
        )
    }
}
