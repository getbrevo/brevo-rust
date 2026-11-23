pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookRequestHeadersItem {
    /// Header key name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Header value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateWebhookRequestHeadersItem {
    pub fn builder() -> CreateWebhookRequestHeadersItemBuilder {
        <CreateWebhookRequestHeadersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookRequestHeadersItemBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl CreateWebhookRequestHeadersItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookRequestHeadersItem`].
    pub fn build(self) -> Result<CreateWebhookRequestHeadersItem, BuildError> {
        Ok(CreateWebhookRequestHeadersItem {
            key: self.key,
            value: self.value,
        })
    }
}
