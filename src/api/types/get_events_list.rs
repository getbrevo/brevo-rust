pub use crate::prelude::*;

/// Response containing a list of events and the total count for pagination
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEventsList {
    /// List of matching events ordered by event_date descending
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GetEventsListEventsItem>>,
    /// Total count of events matching the filters (use for pagination)
    #[serde(default)]
    pub count: i64,
}

impl GetEventsList {
    pub fn builder() -> GetEventsListBuilder {
        <GetEventsListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEventsListBuilder {
    events: Option<Vec<GetEventsListEventsItem>>,
    count: Option<i64>,
}

impl GetEventsListBuilder {
    pub fn events(mut self, value: Vec<GetEventsListEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEventsList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](GetEventsListBuilder::count)
    pub fn build(self) -> Result<GetEventsList, BuildError> {
        Ok(GetEventsList {
            events: self.events,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
