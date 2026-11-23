pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTransacEmailResponse {
    /// Message ID of the transactional email sent
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "messageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ids: Option<Vec<String>>,
}

impl SendTransacEmailResponse {
    pub fn builder() -> SendTransacEmailResponseBuilder {
        <SendTransacEmailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacEmailResponseBuilder {
    message_id: Option<String>,
    message_ids: Option<Vec<String>>,
}

impl SendTransacEmailResponseBuilder {
    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn message_ids(mut self, value: Vec<String>) -> Self {
        self.message_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTransacEmailResponse`].
    pub fn build(self) -> Result<SendTransacEmailResponse, BuildError> {
        Ok(SendTransacEmailResponse {
            message_id: self.message_id,
            message_ids: self.message_ids,
        })
    }
}
