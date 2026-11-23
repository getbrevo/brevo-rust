pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateWhatsAppTemplateRequest {
    /// Body of the template. **Maximum allowed characters are 1024**
    #[serde(rename = "bodyText")]
    #[serde(default)]
    pub body_text: String,
    /// Category of the template
    pub category: CreateWhatsAppTemplateRequestCategory,
    /// Text content of the header in the template.  **Maximum allowed characters are 45** **Use this field to add text content in template header and if mediaUrl is empty**
    #[serde(rename = "headerText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_text: Option<String>,
    /// Language of the template. For Example :
    /// **en** for English
    #[serde(default)]
    pub language: String,
    /// Absolute url of the media file **(no local file)** for the header. **Use this field in you want to add media in Template header and headerText is empty** Allowed extensions for media files are: #### jpeg | png | mp4 | pdf
    #[serde(rename = "mediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// Name of the template
    #[serde(default)]
    pub name: String,
    /// source of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CreateWhatsAppTemplateRequestSource>,
}

impl CreateWhatsAppTemplateRequest {
    pub fn builder() -> CreateWhatsAppTemplateRequestBuilder {
        <CreateWhatsAppTemplateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWhatsAppTemplateRequestBuilder {
    body_text: Option<String>,
    category: Option<CreateWhatsAppTemplateRequestCategory>,
    header_text: Option<String>,
    language: Option<String>,
    media_url: Option<String>,
    name: Option<String>,
    source: Option<CreateWhatsAppTemplateRequestSource>,
}

impl CreateWhatsAppTemplateRequestBuilder {
    pub fn body_text(mut self, value: impl Into<String>) -> Self {
        self.body_text = Some(value.into());
        self
    }

    pub fn category(mut self, value: CreateWhatsAppTemplateRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn header_text(mut self, value: impl Into<String>) -> Self {
        self.header_text = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn media_url(mut self, value: impl Into<String>) -> Self {
        self.media_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn source(mut self, value: CreateWhatsAppTemplateRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWhatsAppTemplateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body_text`](CreateWhatsAppTemplateRequestBuilder::body_text)
    /// - [`category`](CreateWhatsAppTemplateRequestBuilder::category)
    /// - [`language`](CreateWhatsAppTemplateRequestBuilder::language)
    /// - [`name`](CreateWhatsAppTemplateRequestBuilder::name)
    pub fn build(self) -> Result<CreateWhatsAppTemplateRequest, BuildError> {
        Ok(CreateWhatsAppTemplateRequest {
            body_text: self
                .body_text
                .ok_or_else(|| BuildError::missing_field("body_text"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            header_text: self.header_text,
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            media_url: self.media_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            source: self.source,
        })
    }
}
