pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEcommerceAttributionMetricsResponse {
    /// List of conversion attribution metrics
    #[serde(default)]
    pub results: Vec<ConversionSourceMetrics>,
    /// Attribution list aggregated totals
    #[serde(default)]
    pub totals: GetEcommerceAttributionMetricsResponseTotals,
}

impl GetEcommerceAttributionMetricsResponse {
    pub fn builder() -> GetEcommerceAttributionMetricsResponseBuilder {
        <GetEcommerceAttributionMetricsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEcommerceAttributionMetricsResponseBuilder {
    results: Option<Vec<ConversionSourceMetrics>>,
    totals: Option<GetEcommerceAttributionMetricsResponseTotals>,
}

impl GetEcommerceAttributionMetricsResponseBuilder {
    pub fn results(mut self, value: Vec<ConversionSourceMetrics>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn totals(mut self, value: GetEcommerceAttributionMetricsResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEcommerceAttributionMetricsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](GetEcommerceAttributionMetricsResponseBuilder::results)
    /// - [`totals`](GetEcommerceAttributionMetricsResponseBuilder::totals)
    pub fn build(self) -> Result<GetEcommerceAttributionMetricsResponse, BuildError> {
        Ok(GetEcommerceAttributionMetricsResponse {
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
