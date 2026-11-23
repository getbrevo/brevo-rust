pub use crate::prelude::*;

/// Attribution list aggregated totals
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEcommerceAttributionMetricsResponseTotals {
    #[serde(rename = "averageBasket")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub average_basket: f64,
    #[serde(rename = "ordersCount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub orders_count: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub revenue: f64,
}

impl GetEcommerceAttributionMetricsResponseTotals {
    pub fn builder() -> GetEcommerceAttributionMetricsResponseTotalsBuilder {
        <GetEcommerceAttributionMetricsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEcommerceAttributionMetricsResponseTotalsBuilder {
    average_basket: Option<f64>,
    orders_count: Option<f64>,
    revenue: Option<f64>,
}

impl GetEcommerceAttributionMetricsResponseTotalsBuilder {
    pub fn average_basket(mut self, value: f64) -> Self {
        self.average_basket = Some(value);
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

    /// Consumes the builder and constructs a [`GetEcommerceAttributionMetricsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`average_basket`](GetEcommerceAttributionMetricsResponseTotalsBuilder::average_basket)
    /// - [`orders_count`](GetEcommerceAttributionMetricsResponseTotalsBuilder::orders_count)
    /// - [`revenue`](GetEcommerceAttributionMetricsResponseTotalsBuilder::revenue)
    pub fn build(self) -> Result<GetEcommerceAttributionMetricsResponseTotals, BuildError> {
        Ok(GetEcommerceAttributionMetricsResponseTotals {
            average_basket: self
                .average_basket
                .ok_or_else(|| BuildError::missing_field("average_basket"))?,
            orders_count: self
                .orders_count
                .ok_or_else(|| BuildError::missing_field("orders_count"))?,
            revenue: self
                .revenue
                .ok_or_else(|| BuildError::missing_field("revenue"))?,
        })
    }
}
