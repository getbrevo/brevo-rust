pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsTemplatesResponseTemplatesItem {
    /// ID of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Name of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL shortening is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_url_enabled: Option<bool>,
    /// message content for the sms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Manage US compliance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<GetSmsTemplatesResponseTemplatesItemCompliance>,
    /// Enable Unicode encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<bool>,
    /// Support unicode characters in the sms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_sms: Option<bool>,
    /// file name of the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_file: Option<String>,
    /// file size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_file_size: Option<String>,
    /// URL of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// Creation UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Creation UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl GetSmsTemplatesResponseTemplatesItem {
    pub fn builder() -> GetSmsTemplatesResponseTemplatesItemBuilder {
        <GetSmsTemplatesResponseTemplatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsTemplatesResponseTemplatesItemBuilder {
    id: Option<i64>,
    name: Option<String>,
    short_url_enabled: Option<bool>,
    message: Option<String>,
    compliance: Option<GetSmsTemplatesResponseTemplatesItemCompliance>,
    encryption: Option<bool>,
    unicode_sms: Option<bool>,
    media_file: Option<String>,
    media_file_size: Option<String>,
    media_url: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl GetSmsTemplatesResponseTemplatesItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn short_url_enabled(mut self, value: bool) -> Self {
        self.short_url_enabled = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn compliance(mut self, value: GetSmsTemplatesResponseTemplatesItemCompliance) -> Self {
        self.compliance = Some(value);
        self
    }

    pub fn encryption(mut self, value: bool) -> Self {
        self.encryption = Some(value);
        self
    }

    pub fn unicode_sms(mut self, value: bool) -> Self {
        self.unicode_sms = Some(value);
        self
    }

    pub fn media_file(mut self, value: impl Into<String>) -> Self {
        self.media_file = Some(value.into());
        self
    }

    pub fn media_file_size(mut self, value: impl Into<String>) -> Self {
        self.media_file_size = Some(value.into());
        self
    }

    pub fn media_url(mut self, value: impl Into<String>) -> Self {
        self.media_url = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSmsTemplatesResponseTemplatesItem`].
    pub fn build(self) -> Result<GetSmsTemplatesResponseTemplatesItem, BuildError> {
        Ok(GetSmsTemplatesResponseTemplatesItem {
            id: self.id,
            name: self.name,
            short_url_enabled: self.short_url_enabled,
            message: self.message,
            compliance: self.compliance,
            encryption: self.encryption,
            unicode_sms: self.unicode_sms,
            media_file: self.media_file,
            media_file_size: self.media_file_size,
            media_url: self.media_url,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
