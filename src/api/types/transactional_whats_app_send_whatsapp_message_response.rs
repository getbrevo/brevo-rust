pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendWhatsappMessageResponse {
    /// messageId of sent message
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
}

impl SendWhatsappMessageResponse {
    pub fn builder() -> SendWhatsappMessageResponseBuilder {
        <SendWhatsappMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendWhatsappMessageResponseBuilder {
    message_id: Option<String>,
}

impl SendWhatsappMessageResponseBuilder {
    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendWhatsappMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](SendWhatsappMessageResponseBuilder::message_id)
    pub fn build(self) -> Result<SendWhatsappMessageResponse, BuildError> {
        Ok(SendWhatsappMessageResponse {
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
        })
    }
}
