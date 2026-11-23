pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GetSmsEventsResponseEventsItem>>,
}

impl GetSmsEventsResponse {
    pub fn builder() -> GetSmsEventsResponseBuilder {
        <GetSmsEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsEventsResponseBuilder {
    events: Option<Vec<GetSmsEventsResponseEventsItem>>,
}

impl GetSmsEventsResponseBuilder {
    pub fn events(mut self, value: Vec<GetSmsEventsResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsEventsResponse`].
    pub fn build(self) -> Result<GetSmsEventsResponse, BuildError> {
        Ok(GetSmsEventsResponse {
            events: self.events,
        })
    }
}
