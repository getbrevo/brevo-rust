pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSmtpTemplateRequest {
    /// Absolute url of the attachment (**no local file**). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps'
    #[serde(rename = "attachmentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// Body of the message (HTML version). The field must have more than 10 characters. **REQUIRED if htmlUrl is empty**
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// Url which contents the body of the email message. REQUIRED if htmlContent is empty
    #[serde(rename = "htmlUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    /// Status of template. isActive = true means template is active and isActive = false means template is inactive
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Email on which campaign recipients will be able to reply to
    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    #[serde(default)]
    pub sender: CreateSmtpTemplateRequestSender,
    /// Subject of the template
    #[serde(default)]
    pub subject: String,
    /// Tag of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Name of the template
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_field: Option<String>,
}

impl CreateSmtpTemplateRequest {
    pub fn builder() -> CreateSmtpTemplateRequestBuilder {
        <CreateSmtpTemplateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSmtpTemplateRequestBuilder {
    attachment_url: Option<String>,
    html_content: Option<String>,
    html_url: Option<String>,
    is_active: Option<bool>,
    reply_to: Option<String>,
    sender: Option<CreateSmtpTemplateRequestSender>,
    subject: Option<String>,
    tag: Option<String>,
    template_name: Option<String>,
    to_field: Option<String>,
}

impl CreateSmtpTemplateRequestBuilder {
    pub fn attachment_url(mut self, value: impl Into<String>) -> Self {
        self.attachment_url = Some(value.into());
        self
    }

    pub fn html_content(mut self, value: impl Into<String>) -> Self {
        self.html_content = Some(value.into());
        self
    }

    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.html_url = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }

    pub fn sender(mut self, value: CreateSmtpTemplateRequestSender) -> Self {
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

    pub fn template_name(mut self, value: impl Into<String>) -> Self {
        self.template_name = Some(value.into());
        self
    }

    pub fn to_field(mut self, value: impl Into<String>) -> Self {
        self.to_field = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSmtpTemplateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sender`](CreateSmtpTemplateRequestBuilder::sender)
    /// - [`subject`](CreateSmtpTemplateRequestBuilder::subject)
    /// - [`template_name`](CreateSmtpTemplateRequestBuilder::template_name)
    pub fn build(self) -> Result<CreateSmtpTemplateRequest, BuildError> {
        Ok(CreateSmtpTemplateRequest {
            attachment_url: self.attachment_url,
            html_content: self.html_content,
            html_url: self.html_url,
            is_active: self.is_active,
            reply_to: self.reply_to,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            tag: self.tag,
            template_name: self
                .template_name
                .ok_or_else(|| BuildError::missing_field("template_name"))?,
            to_field: self.to_field,
        })
    }
}
