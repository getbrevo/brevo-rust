pub use crate::prelude::*;

/// An object containing details about the email sender (applicable only to messages in email threads).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageFrom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ConversationsMessageFrom {
    pub fn builder() -> ConversationsMessageFromBuilder {
        <ConversationsMessageFromBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageFromBuilder {
    email: Option<String>,
    name: Option<String>,
}

impl ConversationsMessageFromBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageFrom`].
    pub fn build(self) -> Result<ConversationsMessageFrom, BuildError> {
        Ok(ConversationsMessageFrom {
            email: self.email,
            name: self.name,
        })
    }
}
