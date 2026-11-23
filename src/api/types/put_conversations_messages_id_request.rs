pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutConversationsMessagesIdRequest {
    /// The new message text.
    #[serde(default)]
    pub text: String,
}

impl PutConversationsMessagesIdRequest {
    pub fn builder() -> PutConversationsMessagesIdRequestBuilder {
        <PutConversationsMessagesIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutConversationsMessagesIdRequestBuilder {
    text: Option<String>,
}

impl PutConversationsMessagesIdRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PutConversationsMessagesIdRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](PutConversationsMessagesIdRequestBuilder::text)
    pub fn build(self) -> Result<PutConversationsMessagesIdRequest, BuildError> {
        Ok(PutConversationsMessagesIdRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
