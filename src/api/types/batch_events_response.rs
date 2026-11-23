pub use crate::prelude::*;

/// Response returned when a batch of events is processed
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchEventsResponse {
    /// Status of the batch request
    #[serde(default)]
    pub status: String,
    /// Total number of events submitted in the batch
    #[serde(default)]
    pub total_events: i64,
    /// Number of events that were successfully processed
    #[serde(default)]
    pub successful_events: i64,
    /// Number of events that failed to be processed
    #[serde(default)]
    pub failed_events: i64,
    /// List of errors for the failed events
    #[serde(default)]
    pub errors: Vec<BatchEventsResponseErrorsItem>,
}

impl BatchEventsResponse {
    pub fn builder() -> BatchEventsResponseBuilder {
        <BatchEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchEventsResponseBuilder {
    status: Option<String>,
    total_events: Option<i64>,
    successful_events: Option<i64>,
    failed_events: Option<i64>,
    errors: Option<Vec<BatchEventsResponseErrorsItem>>,
}

impl BatchEventsResponseBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn total_events(mut self, value: i64) -> Self {
        self.total_events = Some(value);
        self
    }

    pub fn successful_events(mut self, value: i64) -> Self {
        self.successful_events = Some(value);
        self
    }

    pub fn failed_events(mut self, value: i64) -> Self {
        self.failed_events = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<BatchEventsResponseErrorsItem>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BatchEventsResponseBuilder::status)
    /// - [`total_events`](BatchEventsResponseBuilder::total_events)
    /// - [`successful_events`](BatchEventsResponseBuilder::successful_events)
    /// - [`failed_events`](BatchEventsResponseBuilder::failed_events)
    /// - [`errors`](BatchEventsResponseBuilder::errors)
    pub fn build(self) -> Result<BatchEventsResponse, BuildError> {
        Ok(BatchEventsResponse {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total_events: self
                .total_events
                .ok_or_else(|| BuildError::missing_field("total_events"))?,
            successful_events: self
                .successful_events
                .ok_or_else(|| BuildError::missing_field("successful_events"))?,
            failed_events: self
                .failed_events
                .ok_or_else(|| BuildError::missing_field("failed_events"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
        })
    }
}
