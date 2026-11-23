pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateBatchOrderRequest {
    /// Defines whether you want your orders to be considered as live data or as historical data (import of past data, synchronising data). True: orders will not trigger any automation workflows. False: orders will trigger workflows as usual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
    /// Webhook URL to receive the status of the batch request
    #[serde(rename = "notifyUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// array of order objects
    #[serde(default)]
    pub orders: Vec<Order>,
}

impl CreateBatchOrderRequest {
    pub fn builder() -> CreateBatchOrderRequestBuilder {
        <CreateBatchOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchOrderRequestBuilder {
    historical: Option<bool>,
    notify_url: Option<String>,
    orders: Option<Vec<Order>>,
}

impl CreateBatchOrderRequestBuilder {
    pub fn historical(mut self, value: bool) -> Self {
        self.historical = Some(value);
        self
    }

    pub fn notify_url(mut self, value: impl Into<String>) -> Self {
        self.notify_url = Some(value.into());
        self
    }

    pub fn orders(mut self, value: Vec<Order>) -> Self {
        self.orders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`orders`](CreateBatchOrderRequestBuilder::orders)
    pub fn build(self) -> Result<CreateBatchOrderRequest, BuildError> {
        Ok(CreateBatchOrderRequest {
            historical: self.historical,
            notify_url: self.notify_url,
            orders: self
                .orders
                .ok_or_else(|| BuildError::missing_field("orders"))?,
        })
    }
}
