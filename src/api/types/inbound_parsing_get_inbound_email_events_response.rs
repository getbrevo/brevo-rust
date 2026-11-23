pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInboundEmailEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GetInboundEmailEventsResponseEventsItem>>,
}

impl GetInboundEmailEventsResponse {
    pub fn builder() -> GetInboundEmailEventsResponseBuilder {
        <GetInboundEmailEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInboundEmailEventsResponseBuilder {
    events: Option<Vec<GetInboundEmailEventsResponseEventsItem>>,
}

impl GetInboundEmailEventsResponseBuilder {
    pub fn events(mut self, value: Vec<GetInboundEmailEventsResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetInboundEmailEventsResponse`].
    pub fn build(self) -> Result<GetInboundEmailEventsResponse, BuildError> {
        Ok(GetInboundEmailEventsResponse {
            events: self.events,
        })
    }
}
