pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageToItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ConversationsMessageToItem {
    pub fn builder() -> ConversationsMessageToItemBuilder {
        <ConversationsMessageToItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageToItemBuilder {
    email: Option<String>,
    name: Option<String>,
}

impl ConversationsMessageToItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageToItem`].
    pub fn build(self) -> Result<ConversationsMessageToItem, BuildError> {
        Ok(ConversationsMessageToItem {
            email: self.email,
            name: self.name,
        })
    }
}
