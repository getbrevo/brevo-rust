pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTransacEmailRequestAttachmentItem {
    /// Base64-encoded attachment data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Attachment filename. Required when `content` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Absolute URL of the attachment. Local file paths are not supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SendTransacEmailRequestAttachmentItem {
    pub fn builder() -> SendTransacEmailRequestAttachmentItemBuilder {
        <SendTransacEmailRequestAttachmentItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacEmailRequestAttachmentItemBuilder {
    content: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

impl SendTransacEmailRequestAttachmentItemBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendTransacEmailRequestAttachmentItem`].
    pub fn build(self) -> Result<SendTransacEmailRequestAttachmentItem, BuildError> {
        Ok(SendTransacEmailRequestAttachmentItem {
            content: self.content,
            name: self.name,
            url: self.url,
        })
    }
}
