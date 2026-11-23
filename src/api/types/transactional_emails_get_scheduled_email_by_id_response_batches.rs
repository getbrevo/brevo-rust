pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetScheduledEmailByIdResponseBatches {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batches: Option<Vec<GetScheduledEmailByIdResponseBatchesBatchesItem>>,
    /// Total number of batches
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl GetScheduledEmailByIdResponseBatches {
    pub fn builder() -> GetScheduledEmailByIdResponseBatchesBuilder {
        <GetScheduledEmailByIdResponseBatchesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetScheduledEmailByIdResponseBatchesBuilder {
    batches: Option<Vec<GetScheduledEmailByIdResponseBatchesBatchesItem>>,
    count: Option<i64>,
}

impl GetScheduledEmailByIdResponseBatchesBuilder {
    pub fn batches(mut self, value: Vec<GetScheduledEmailByIdResponseBatchesBatchesItem>) -> Self {
        self.batches = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetScheduledEmailByIdResponseBatches`].
    pub fn build(self) -> Result<GetScheduledEmailByIdResponseBatches, BuildError> {
        Ok(GetScheduledEmailByIdResponseBatches {
            batches: self.batches,
            count: self.count,
        })
    }
}
