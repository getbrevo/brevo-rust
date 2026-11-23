pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageBccItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ConversationsMessageBccItem {
    pub fn builder() -> ConversationsMessageBccItemBuilder {
        <ConversationsMessageBccItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageBccItemBuilder {
    email: Option<String>,
    name: Option<String>,
}

impl ConversationsMessageBccItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageBccItem`].
    pub fn build(self) -> Result<ConversationsMessageBccItem, BuildError> {
        Ok(ConversationsMessageBccItem {
            email: self.email,
            name: self.name,
        })
    }
}
