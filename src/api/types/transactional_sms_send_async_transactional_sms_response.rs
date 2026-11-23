pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendAsyncTransactionalSmsResponse {
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: i64,
}

impl SendAsyncTransactionalSmsResponse {
    pub fn builder() -> SendAsyncTransactionalSmsResponseBuilder {
        <SendAsyncTransactionalSmsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendAsyncTransactionalSmsResponseBuilder {
    message_id: Option<i64>,
}

impl SendAsyncTransactionalSmsResponseBuilder {
    pub fn message_id(mut self, value: i64) -> Self {
        self.message_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendAsyncTransactionalSmsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](SendAsyncTransactionalSmsResponseBuilder::message_id)
    pub fn build(self) -> Result<SendAsyncTransactionalSmsResponse, BuildError> {
        Ok(SendAsyncTransactionalSmsResponse {
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
        })
    }
}
