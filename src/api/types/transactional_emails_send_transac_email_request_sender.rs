pub use crate::prelude::*;

/// Sender information. Required when `templateId` is not provided. Specify either an email address (with optional name) or a sender ID. The `name` field is ignored when `id` is provided.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTransacEmailRequestSender {
    /// Sender email address. Required when `id` is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Sender identifier. Required when `email` is not provided. Use this field to select a sender with a specific IP pool (dedicated IP users only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Display name of the sender. Maximum length is 70 characters. Only applicable when `email` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SendTransacEmailRequestSender {
    pub fn builder() -> SendTransacEmailRequestSenderBuilder {
        <SendTransacEmailRequestSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacEmailRequestSenderBuilder {
    email: Option<String>,
    id: Option<i64>,
    name: Option<String>,
}

impl SendTransacEmailRequestSenderBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendTransacEmailRequestSender`].
    pub fn build(self) -> Result<SendTransacEmailRequestSender, BuildError> {
        Ok(SendTransacEmailRequestSender {
            email: self.email,
            id: self.id,
            name: self.name,
        })
    }
}
