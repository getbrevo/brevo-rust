pub use crate::prelude::*;

/// Status of the message forwarding to the source.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageForwardedToSourceStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
}

impl ConversationsMessageForwardedToSourceStatus {
    pub fn builder() -> ConversationsMessageForwardedToSourceStatusBuilder {
        <ConversationsMessageForwardedToSourceStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageForwardedToSourceStatusBuilder {
    error: Option<String>,
    is_success: Option<bool>,
}

impl ConversationsMessageForwardedToSourceStatusBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn is_success(mut self, value: bool) -> Self {
        self.is_success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageForwardedToSourceStatus`].
    pub fn build(self) -> Result<ConversationsMessageForwardedToSourceStatus, BuildError> {
        Ok(ConversationsMessageForwardedToSourceStatus {
            error: self.error,
            is_success: self.is_success,
        })
    }
}
