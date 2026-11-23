pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageFile {
    /// Name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// MIME type of the file.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Whether the file is an image.
    #[serde(rename = "isImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_image: Option<bool>,
    /// Whether the file is a sticker.
    #[serde(rename = "isSticker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sticker: Option<bool>,
    /// Whether the file is inline.
    #[serde(rename = "isInline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inline: Option<bool>,
    /// The ID of the inline file.
    #[serde(rename = "inlineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_id: Option<String>,
    /// Size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Image info, present when the file is an image.
    #[serde(rename = "imageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_info: Option<ConversationsMessageFileImageInfo>,
    /// Whether the file extension is allowed for download.
    #[serde(rename = "isAllowedFileType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_allowed_file_type: Option<bool>,
}

impl ConversationsMessageFile {
    pub fn builder() -> ConversationsMessageFileBuilder {
        <ConversationsMessageFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageFileBuilder {
    name: Option<String>,
    link: Option<String>,
    mime_type: Option<String>,
    is_image: Option<bool>,
    is_sticker: Option<bool>,
    is_inline: Option<bool>,
    inline_id: Option<String>,
    size: Option<i64>,
    image_info: Option<ConversationsMessageFileImageInfo>,
    is_allowed_file_type: Option<bool>,
}

impl ConversationsMessageFileBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn is_image(mut self, value: bool) -> Self {
        self.is_image = Some(value);
        self
    }

    pub fn is_sticker(mut self, value: bool) -> Self {
        self.is_sticker = Some(value);
        self
    }

    pub fn is_inline(mut self, value: bool) -> Self {
        self.is_inline = Some(value);
        self
    }

    pub fn inline_id(mut self, value: impl Into<String>) -> Self {
        self.inline_id = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn image_info(mut self, value: ConversationsMessageFileImageInfo) -> Self {
        self.image_info = Some(value);
        self
    }

    pub fn is_allowed_file_type(mut self, value: bool) -> Self {
        self.is_allowed_file_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageFile`].
    pub fn build(self) -> Result<ConversationsMessageFile, BuildError> {
        Ok(ConversationsMessageFile {
            name: self.name,
            link: self.link,
            mime_type: self.mime_type,
            is_image: self.is_image,
            is_sticker: self.is_sticker,
            is_inline: self.is_inline,
            inline_id: self.inline_id,
            size: self.size,
            image_info: self.image_info,
            is_allowed_file_type: self.is_allowed_file_type,
        })
    }
}
