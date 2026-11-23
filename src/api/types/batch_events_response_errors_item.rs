pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchEventsResponseErrorsItem {
    /// Index positions (0-based) of the events that caused the error
    #[serde(rename = "eventIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_index: Option<Vec<i64>>,
    /// Description of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BatchEventsResponseErrorsItem {
    pub fn builder() -> BatchEventsResponseErrorsItemBuilder {
        <BatchEventsResponseErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchEventsResponseErrorsItemBuilder {
    event_index: Option<Vec<i64>>,
    message: Option<String>,
}

impl BatchEventsResponseErrorsItemBuilder {
    pub fn event_index(mut self, value: Vec<i64>) -> Self {
        self.event_index = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchEventsResponseErrorsItem`].
    pub fn build(self) -> Result<BatchEventsResponseErrorsItem, BuildError> {
        Ok(BatchEventsResponseErrorsItem {
            event_index: self.event_index,
            message: self.message,
        })
    }
}
