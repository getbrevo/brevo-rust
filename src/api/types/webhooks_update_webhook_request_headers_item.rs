pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhookRequestHeadersItem {
    /// Header key name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Header value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateWebhookRequestHeadersItem {
    pub fn builder() -> UpdateWebhookRequestHeadersItemBuilder {
        <UpdateWebhookRequestHeadersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhookRequestHeadersItemBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl UpdateWebhookRequestHeadersItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhookRequestHeadersItem`].
    pub fn build(self) -> Result<UpdateWebhookRequestHeadersItem, BuildError> {
        Ok(UpdateWebhookRequestHeadersItem {
            key: self.key,
            value: self.value,
        })
    }
}
