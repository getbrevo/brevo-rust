pub use crate::prelude::*;

/// Response returned when all events in the batch are accepted for async processing
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchAcceptedResponse {
    /// Confirmation message indicating the batch was accepted
    #[serde(default)]
    pub message: String,
    /// Number of events queued for processing
    #[serde(default)]
    pub count: i64,
}

impl BatchAcceptedResponse {
    pub fn builder() -> BatchAcceptedResponseBuilder {
        <BatchAcceptedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchAcceptedResponseBuilder {
    message: Option<String>,
    count: Option<i64>,
}

impl BatchAcceptedResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchAcceptedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](BatchAcceptedResponseBuilder::message)
    /// - [`count`](BatchAcceptedResponseBuilder::count)
    pub fn build(self) -> Result<BatchAcceptedResponse, BuildError> {
        Ok(BatchAcceptedResponse {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
