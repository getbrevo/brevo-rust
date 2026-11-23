pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutConversationsPushedMessagesIdRequest {
    /// The new message text.
    #[serde(default)]
    pub text: String,
}

impl PutConversationsPushedMessagesIdRequest {
    pub fn builder() -> PutConversationsPushedMessagesIdRequestBuilder {
        <PutConversationsPushedMessagesIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutConversationsPushedMessagesIdRequestBuilder {
    text: Option<String>,
}

impl PutConversationsPushedMessagesIdRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PutConversationsPushedMessagesIdRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](PutConversationsPushedMessagesIdRequestBuilder::text)
    pub fn build(self) -> Result<PutConversationsPushedMessagesIdRequest, BuildError> {
        Ok(PutConversationsPushedMessagesIdRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
