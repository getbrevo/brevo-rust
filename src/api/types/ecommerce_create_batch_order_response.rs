pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateBatchOrderResponse {
    /// Number of orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Batch ID of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_id: Option<f64>,
}

impl CreateBatchOrderResponse {
    pub fn builder() -> CreateBatchOrderResponseBuilder {
        <CreateBatchOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchOrderResponseBuilder {
    count: Option<i64>,
    batch_id: Option<f64>,
}

impl CreateBatchOrderResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn batch_id(mut self, value: f64) -> Self {
        self.batch_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchOrderResponse`].
    pub fn build(self) -> Result<CreateBatchOrderResponse, BuildError> {
        Ok(CreateBatchOrderResponse {
            count: self.count,
            batch_id: self.batch_id,
        })
    }
}
