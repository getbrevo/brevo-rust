pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateBatchEventsRequest {
    #[serde(default)]
    pub events: Vec<CreateBatchEventsRequestEventsItem>,
}

impl CreateBatchEventsRequest {
    pub fn builder() -> CreateBatchEventsRequestBuilder {
        <CreateBatchEventsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchEventsRequestBuilder {
    events: Option<Vec<CreateBatchEventsRequestEventsItem>>,
}

impl CreateBatchEventsRequestBuilder {
    pub fn events(mut self, value: Vec<CreateBatchEventsRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchEventsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](CreateBatchEventsRequestBuilder::events)
    pub fn build(self) -> Result<CreateBatchEventsRequest, BuildError> {
        Ok(CreateBatchEventsRequest {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
        })
    }
}
