pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversionSourceMetrics {
    #[serde(rename = "averageBasket")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub average_basket: f64,
    #[serde(rename = "conversionSource")]
    pub conversion_source: ConversionSourceMetricsConversionSource,
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
}

impl ConversionSourceMetrics {
    pub fn builder() -> ConversionSourceMetricsBuilder {
        <ConversionSourceMetricsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversionSourceMetricsBuilder {
    average_basket: Option<f64>,
    conversion_source: Option<ConversionSourceMetricsConversionSource>,
    id: Option<f64>,
    orders_count: Option<f64>,
    revenue: Option<f64>,
}

impl ConversionSourceMetricsBuilder {
    pub fn average_basket(mut self, value: f64) -> Self {
        self.average_basket = Some(value);
        self
    }

    pub fn conversion_source(mut self, value: ConversionSourceMetricsConversionSource) -> Self {
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

    /// Consumes the builder and constructs a [`ConversionSourceMetrics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`average_basket`](ConversionSourceMetricsBuilder::average_basket)
    /// - [`conversion_source`](ConversionSourceMetricsBuilder::conversion_source)
    /// - [`id`](ConversionSourceMetricsBuilder::id)
    /// - [`orders_count`](ConversionSourceMetricsBuilder::orders_count)
    /// - [`revenue`](ConversionSourceMetricsBuilder::revenue)
    pub fn build(self) -> Result<ConversionSourceMetrics, BuildError> {
        Ok(ConversionSourceMetrics {
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
        })
    }
}
