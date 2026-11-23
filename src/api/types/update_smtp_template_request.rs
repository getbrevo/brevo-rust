pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSmtpTemplateRequest {
    /// Absolute url of the attachment (**no local file**). Extensions allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    #[serde(rename = "attachmentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// **Required if htmlUrl is empty**. If the template is designed using Drag & Drop editor via HTML content, then the design page will not have Drag & Drop editor access for that template. Body of the message (HTML must have more than 10 characters)
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// **Required if htmlContent is empty**. URL to the body of the email (HTML)
    #[serde(rename = "htmlUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    /// Status of the template. isActive = false means template is inactive, isActive = true means template is active
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Email on which campaign recipients will be able to reply to
    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<UpdateSmtpTemplateRequestSender>,
    /// Subject of the email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Tag of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Name of the template
    #[serde(rename = "templateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_field: Option<String>,
}

impl UpdateSmtpTemplateRequest {
    pub fn builder() -> UpdateSmtpTemplateRequestBuilder {
        <UpdateSmtpTemplateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSmtpTemplateRequestBuilder {
    attachment_url: Option<String>,
    html_content: Option<String>,
    html_url: Option<String>,
    is_active: Option<bool>,
    reply_to: Option<String>,
    sender: Option<UpdateSmtpTemplateRequestSender>,
    subject: Option<String>,
    tag: Option<String>,
    template_name: Option<String>,
    to_field: Option<String>,
}

impl UpdateSmtpTemplateRequestBuilder {
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

    pub fn sender(mut self, value: UpdateSmtpTemplateRequestSender) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateSmtpTemplateRequest`].
    pub fn build(self) -> Result<UpdateSmtpTemplateRequest, BuildError> {
        Ok(UpdateSmtpTemplateRequest {
            attachment_url: self.attachment_url,
            html_content: self.html_content,
            html_url: self.html_url,
            is_active: self.is_active,
            reply_to: self.reply_to,
            sender: self.sender,
            subject: self.subject,
            tag: self.tag,
            template_name: self.template_name,
            to_field: self.to_field,
        })
    }
}
