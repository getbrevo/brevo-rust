pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmtpTemplateOverview {
    /// Creation UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// It is true if template is a valid Double opt-in (DOI) template, otherwise it is false. This field will be available only in case of single template detail call.
    #[serde(rename = "doiTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi_template: Option<bool>,
    /// HTML content of the template
    #[serde(rename = "htmlContent")]
    #[serde(default)]
    pub html_content: String,
    /// ID of the template
    #[serde(default)]
    pub id: i64,
    /// Status of template (true=active, false=inactive)
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    /// Last modification UTC date-time of the template (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Name of the template
    #[serde(default)]
    pub name: String,
    /// Email defined as the "Reply to" for the template
    #[serde(rename = "replyTo")]
    #[serde(default)]
    pub reply_to: String,
    #[serde(default)]
    pub sender: GetSmtpTemplateOverviewSender,
    /// Subject of the template
    #[serde(default)]
    pub subject: String,
    /// Tag of the template
    #[serde(default)]
    pub tag: String,
    /// Status of test sending for the template (true=test email has been sent, false=test email has not been sent)
    #[serde(rename = "testSent")]
    #[serde(default)]
    pub test_sent: bool,
    /// Customisation of the "to" field for the template
    #[serde(rename = "toField")]
    #[serde(default)]
    pub to_field: String,
    /// Custom template identifier, if one was assigned during template creation. Only present when the template has a custom ID.
    #[serde(rename = "customTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_template_id: Option<String>,
}

impl GetSmtpTemplateOverview {
    pub fn builder() -> GetSmtpTemplateOverviewBuilder {
        <GetSmtpTemplateOverviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmtpTemplateOverviewBuilder {
    created_at: Option<String>,
    doi_template: Option<bool>,
    html_content: Option<String>,
    id: Option<i64>,
    is_active: Option<bool>,
    modified_at: Option<String>,
    name: Option<String>,
    reply_to: Option<String>,
    sender: Option<GetSmtpTemplateOverviewSender>,
    subject: Option<String>,
    tag: Option<String>,
    test_sent: Option<bool>,
    to_field: Option<String>,
    custom_template_id: Option<String>,
}

impl GetSmtpTemplateOverviewBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn doi_template(mut self, value: bool) -> Self {
        self.doi_template = Some(value);
        self
    }

    pub fn html_content(mut self, value: impl Into<String>) -> Self {
        self.html_content = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
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

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }

    pub fn sender(mut self, value: GetSmtpTemplateOverviewSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn test_sent(mut self, value: bool) -> Self {
        self.test_sent = Some(value);
        self
    }

    pub fn to_field(mut self, value: impl Into<String>) -> Self {
        self.to_field = Some(value.into());
        self
    }

    pub fn custom_template_id(mut self, value: impl Into<String>) -> Self {
        self.custom_template_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSmtpTemplateOverview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetSmtpTemplateOverviewBuilder::created_at)
    /// - [`html_content`](GetSmtpTemplateOverviewBuilder::html_content)
    /// - [`id`](GetSmtpTemplateOverviewBuilder::id)
    /// - [`is_active`](GetSmtpTemplateOverviewBuilder::is_active)
    /// - [`modified_at`](GetSmtpTemplateOverviewBuilder::modified_at)
    /// - [`name`](GetSmtpTemplateOverviewBuilder::name)
    /// - [`reply_to`](GetSmtpTemplateOverviewBuilder::reply_to)
    /// - [`sender`](GetSmtpTemplateOverviewBuilder::sender)
    /// - [`subject`](GetSmtpTemplateOverviewBuilder::subject)
    /// - [`tag`](GetSmtpTemplateOverviewBuilder::tag)
    /// - [`test_sent`](GetSmtpTemplateOverviewBuilder::test_sent)
    /// - [`to_field`](GetSmtpTemplateOverviewBuilder::to_field)
    pub fn build(self) -> Result<GetSmtpTemplateOverview, BuildError> {
        Ok(GetSmtpTemplateOverview {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            doi_template: self.doi_template,
            html_content: self
                .html_content
                .ok_or_else(|| BuildError::missing_field("html_content"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            reply_to: self
                .reply_to
                .ok_or_else(|| BuildError::missing_field("reply_to"))?,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            tag: self.tag.ok_or_else(|| BuildError::missing_field("tag"))?,
            test_sent: self
                .test_sent
                .ok_or_else(|| BuildError::missing_field("test_sent"))?,
            to_field: self
                .to_field
                .ok_or_else(|| BuildError::missing_field("to_field"))?,
            custom_template_id: self.custom_template_id,
        })
    }
}
