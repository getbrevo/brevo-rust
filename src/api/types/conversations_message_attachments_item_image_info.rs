pub use crate::prelude::*;

/// Image information (only present for image files).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageAttachmentsItemImageInfo {
    /// Width of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Height of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// URL of the image preview.
    #[serde(rename = "previewLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_link: Option<String>,
}

impl ConversationsMessageAttachmentsItemImageInfo {
    pub fn builder() -> ConversationsMessageAttachmentsItemImageInfoBuilder {
        <ConversationsMessageAttachmentsItemImageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageAttachmentsItemImageInfoBuilder {
    width: Option<i64>,
    height: Option<i64>,
    preview_link: Option<String>,
}

impl ConversationsMessageAttachmentsItemImageInfoBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn preview_link(mut self, value: impl Into<String>) -> Self {
        self.preview_link = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageAttachmentsItemImageInfo`].
    pub fn build(self) -> Result<ConversationsMessageAttachmentsItemImageInfo, BuildError> {
        Ok(ConversationsMessageAttachmentsItemImageInfo {
            width: self.width,
            height: self.height,
            preview_link: self.preview_link,
        })
    }
}
