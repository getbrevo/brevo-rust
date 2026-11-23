pub use crate::prelude::*;

/// An object containing details of the reply-to email address (applicable only to messages in email threads).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageReplyTo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ConversationsMessageReplyTo {
    pub fn builder() -> ConversationsMessageReplyToBuilder {
        <ConversationsMessageReplyToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageReplyToBuilder {
    email: Option<String>,
    name: Option<String>,
}

impl ConversationsMessageReplyToBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageReplyTo`].
    pub fn build(self) -> Result<ConversationsMessageReplyTo, BuildError> {
        Ok(ConversationsMessageReplyTo {
            email: self.email,
            name: self.name,
        })
    }
}
