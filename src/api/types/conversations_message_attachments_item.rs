pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsMessageAttachmentsItem {
    /// The name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The MIME type of the file.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// `true` for images.
    #[serde(rename = "isImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_image: Option<bool>,
    /// `true` for sticker files.
    #[serde(rename = "isSticker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sticker: Option<bool>,
    /// `true` for inline files.
    #[serde(rename = "isInline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inline: Option<bool>,
    /// The ID of the inline file.
    #[serde(rename = "inlineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_id: Option<String>,
    /// The size of the file in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Image information (only present for image files).
    #[serde(rename = "imageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_info: Option<ConversationsMessageAttachmentsItemImageInfo>,
    /// Whether the file extension is allowed for download.
    #[serde(rename = "isAllowedFileType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_allowed_file_type: Option<bool>,
}

impl ConversationsMessageAttachmentsItem {
    pub fn builder() -> ConversationsMessageAttachmentsItemBuilder {
        <ConversationsMessageAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsMessageAttachmentsItemBuilder {
    name: Option<String>,
    link: Option<String>,
    mime_type: Option<String>,
    is_image: Option<bool>,
    is_sticker: Option<bool>,
    is_inline: Option<bool>,
    inline_id: Option<String>,
    size: Option<i64>,
    image_info: Option<ConversationsMessageAttachmentsItemImageInfo>,
    is_allowed_file_type: Option<bool>,
}

impl ConversationsMessageAttachmentsItemBuilder {
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

    pub fn image_info(mut self, value: ConversationsMessageAttachmentsItemImageInfo) -> Self {
        self.image_info = Some(value);
        self
    }

    pub fn is_allowed_file_type(mut self, value: bool) -> Self {
        self.is_allowed_file_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsMessageAttachmentsItem`].
    pub fn build(self) -> Result<ConversationsMessageAttachmentsItem, BuildError> {
        Ok(ConversationsMessageAttachmentsItem {
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
