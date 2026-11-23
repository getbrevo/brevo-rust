pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppTemplatesResponseTemplatesItem {
    /// category of the template
    #[serde(default)]
    pub category: String,
    /// Creation UTC date-time of the whatsApp template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Error reason in the template creation
    #[serde(rename = "errorReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    /// ID of the whatsApp template
    #[serde(default)]
    pub id: i64,
    /// Language in which template exists
    #[serde(default)]
    pub language: String,
    /// UTC date-time of last modification of the whatsApp template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Name of the whatsApp template
    #[serde(default)]
    pub name: String,
    /// Status of the whatsApp template
    #[serde(default)]
    pub status: String,
    /// Type of the whatsApp template
    #[serde(default)]
    pub r#type: String,
}

impl GetWhatsAppTemplatesResponseTemplatesItem {
    pub fn builder() -> GetWhatsAppTemplatesResponseTemplatesItemBuilder {
        <GetWhatsAppTemplatesResponseTemplatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppTemplatesResponseTemplatesItemBuilder {
    category: Option<String>,
    created_at: Option<String>,
    error_reason: Option<String>,
    id: Option<i64>,
    language: Option<String>,
    modified_at: Option<String>,
    name: Option<String>,
    status: Option<String>,
    r#type: Option<String>,
}

impl GetWhatsAppTemplatesResponseTemplatesItemBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn error_reason(mut self, value: impl Into<String>) -> Self {
        self.error_reason = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppTemplatesResponseTemplatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::category)
    /// - [`created_at`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::created_at)
    /// - [`id`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::id)
    /// - [`language`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::language)
    /// - [`modified_at`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::modified_at)
    /// - [`name`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::name)
    /// - [`status`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::status)
    /// - [`r#type`](GetWhatsAppTemplatesResponseTemplatesItemBuilder::r#type)
    pub fn build(self) -> Result<GetWhatsAppTemplatesResponseTemplatesItem, BuildError> {
        Ok(GetWhatsAppTemplatesResponseTemplatesItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            error_reason: self.error_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
