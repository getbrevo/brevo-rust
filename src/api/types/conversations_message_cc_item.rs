pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageCcItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ConversationsMessageCcItem {
    pub fn builder() -> ConversationsMessageCcItemBuilder {
        <ConversationsMessageCcItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageCcItemBuilder {
    email: Option<String>,
    name: Option<String>,
}

impl ConversationsMessageCcItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageCcItem`].
    pub fn build(self) -> Result<ConversationsMessageCcItem, BuildError> {
        Ok(ConversationsMessageCcItem {
            email: self.email,
            name: self.name,
        })
    }
}
