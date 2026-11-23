pub use crate::prelude::*;

/// Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTransacEmailRequestMessageVersionsItemReplyTo {
    /// Email address in reply to
    #[serde(default)]
    pub email: String,
    /// Display name for the reply-to address. Maximum length is 70 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SendTransacEmailRequestMessageVersionsItemReplyTo {
    pub fn builder() -> SendTransacEmailRequestMessageVersionsItemReplyToBuilder {
        <SendTransacEmailRequestMessageVersionsItemReplyToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacEmailRequestMessageVersionsItemReplyToBuilder {
    email: Option<String>,
    name: Option<String>,
}

impl SendTransacEmailRequestMessageVersionsItemReplyToBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendTransacEmailRequestMessageVersionsItemReplyTo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](SendTransacEmailRequestMessageVersionsItemReplyToBuilder::email)
    pub fn build(self) -> Result<SendTransacEmailRequestMessageVersionsItemReplyTo, BuildError> {
        Ok(SendTransacEmailRequestMessageVersionsItemReplyTo {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
        })
    }
}
