pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostPreviewSmtpEmailTemplatesResponse {
    /// Sender email address
    #[serde(rename = "fromEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email: Option<String>,
    /// Sender Name
    #[serde(rename = "fromName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Html content of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Preview text of the template
    #[serde(rename = "previewText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// subject of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "usedFeedNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_feed_names: Option<Vec<String>>,
}

impl PostPreviewSmtpEmailTemplatesResponse {
    pub fn builder() -> PostPreviewSmtpEmailTemplatesResponseBuilder {
        <PostPreviewSmtpEmailTemplatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPreviewSmtpEmailTemplatesResponseBuilder {
    from_email: Option<String>,
    from_name: Option<String>,
    html: Option<String>,
    preview_text: Option<String>,
    subject: Option<String>,
    used_feed_names: Option<Vec<String>>,
}

impl PostPreviewSmtpEmailTemplatesResponseBuilder {
    pub fn from_email(mut self, value: impl Into<String>) -> Self {
        self.from_email = Some(value.into());
        self
    }

    pub fn from_name(mut self, value: impl Into<String>) -> Self {
        self.from_name = Some(value.into());
        self
    }

    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn preview_text(mut self, value: impl Into<String>) -> Self {
        self.preview_text = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn used_feed_names(mut self, value: Vec<String>) -> Self {
        self.used_feed_names = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPreviewSmtpEmailTemplatesResponse`].
    pub fn build(self) -> Result<PostPreviewSmtpEmailTemplatesResponse, BuildError> {
        Ok(PostPreviewSmtpEmailTemplatesResponse {
            from_email: self.from_email,
            from_name: self.from_name,
            html: self.html,
            preview_text: self.preview_text,
            subject: self.subject,
            used_feed_names: self.used_feed_names,
        })
    }
}
