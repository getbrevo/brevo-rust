pub use crate::prelude::*;

/// Image info, present when the file is an image.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageFileImageInfo {
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

impl ConversationsMessageFileImageInfo {
    pub fn builder() -> ConversationsMessageFileImageInfoBuilder {
        <ConversationsMessageFileImageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageFileImageInfoBuilder {
    width: Option<i64>,
    height: Option<i64>,
    preview_link: Option<String>,
}

impl ConversationsMessageFileImageInfoBuilder {
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

    /// Consumes the builder and constructs a [`ConversationsMessageFileImageInfo`].
    pub fn build(self) -> Result<ConversationsMessageFileImageInfo, BuildError> {
        Ok(ConversationsMessageFileImageInfo {
            width: self.width,
            height: self.height,
            preview_link: self.preview_link,
        })
    }
}
