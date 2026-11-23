pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendTransacEmailRequest {
    /// Array of attachment objects. Each attachment must include either an absolute URL (no local file paths) or base64-encoded content, along with the attachment filename. The `name` field is required when `content` is provided. Supported file extensions: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub, eps, odt, mp3, m4a, m4v, wma, ogg, flac, wav, aif, aifc, aiff, mp4, mov, avi, mkv, mpeg, mpg, wmv, pkpass, xlsm. When `templateId` is specified: if the template uses the New Template Language format, both `url` and `content` attachment types are supported; if the template uses the Old Template Language format, the `attachment` parameter is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<SendTransacEmailRequestAttachmentItem>>,
    /// UUIDv4 identifier for the scheduled batch of transactional emails. If omitted, a valid UUIDv4 batch identifier is automatically generated.
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    /// Array of BCC recipient objects. Each object contains an email address and an optional name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<SendTransacEmailRequestBccItem>>,
    /// Array of CC recipient objects. Each object contains an email address and an optional name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<SendTransacEmailRequestCcItem>>,
    /// Custom email headers (non-standard headers) to include in the email. The `sender.ip` header can be set to specify the IP address used for sending transactional emails (dedicated IP users only). Header names must use Title-Case-Format (words separated by hyphens with the first letter of each word capitalized). Headers not in this format are automatically converted. Standard email headers are not supported. Example: `{"sender.ip":"1.2.3.4", "X-Mailin-custom":"some_custom_value", "IdempotencyKey":"abc-123"}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, serde_json::Value>>,
    /// HTML body content of the email. Required when `templateId` is not provided. Ignored when `templateId` is provided.
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// Array of message version objects for sending customized email variants. The `templateId` can be customized per version only if a global `templateId` is provided. The `htmlContent` and `textContent` can be customized per version only if at least one of these is present in the global parameters. Global parameters such as `to` (required), `bcc`, `cc`, `replyTo`, and `subject` can be customized per version. Maximum total recipients per API request is 2000. Maximum recipients per message version is 99. Individual `params` objects must not exceed 100 KB. Cumulative `params` across all versions must not exceed 1000 KB. See https://developers.brevo.com/docs/batch-send-transactional-emails for detailed usage instructions.
    #[serde(rename = "messageVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_versions: Option<Vec<SendTransacEmailRequestMessageVersionsItem>>,
    /// Key-value pairs for template variable substitution. Only applicable when the template uses the New Template Language format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    /// Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<SendTransacEmailRequestReplyTo>,
    /// UTC date-time when the email should be sent (format: YYYY-MM-DDTHH:mm:ss.SSSZ). Include timezone information in the date-time value. Scheduled emails may be delayed by up to 5 minutes.
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub scheduled_at: Option<DateTime<FixedOffset>>,
    /// Sender information. Required when `templateId` is not provided. Specify either an email address (with optional name) or a sender ID. The `name` field is ignored when `id` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<SendTransacEmailRequestSender>,
    /// Email subject line. Required when `templateId` is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Array of tags for categorizing and filtering emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Template identifier
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// Plain text body content of the email. Ignored when `templateId` is provided.
    #[serde(rename = "textContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<String>,
    /// Array of recipient objects. Each object contains an email address and an optional display name. Required when `messageVersions` is not provided. Ignored when `messageVersions` is provided. Example: `[{"name":"Jimmy", "email":"jimmy@example.com"}, {"name":"Joe", "email":"joe@example.com"}]`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<SendTransacEmailRequestToItem>>,
}

impl SendTransacEmailRequest {
    pub fn builder() -> SendTransacEmailRequestBuilder {
        <SendTransacEmailRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacEmailRequestBuilder {
    attachment: Option<Vec<SendTransacEmailRequestAttachmentItem>>,
    batch_id: Option<String>,
    bcc: Option<Vec<SendTransacEmailRequestBccItem>>,
    cc: Option<Vec<SendTransacEmailRequestCcItem>>,
    headers: Option<HashMap<String, serde_json::Value>>,
    html_content: Option<String>,
    message_versions: Option<Vec<SendTransacEmailRequestMessageVersionsItem>>,
    params: Option<HashMap<String, serde_json::Value>>,
    reply_to: Option<SendTransacEmailRequestReplyTo>,
    scheduled_at: Option<DateTime<FixedOffset>>,
    sender: Option<SendTransacEmailRequestSender>,
    subject: Option<String>,
    tags: Option<Vec<String>>,
    template_id: Option<i64>,
    text_content: Option<String>,
    to: Option<Vec<SendTransacEmailRequestToItem>>,
}

impl SendTransacEmailRequestBuilder {
    pub fn attachment(mut self, value: Vec<SendTransacEmailRequestAttachmentItem>) -> Self {
        self.attachment = Some(value);
        self
    }

    pub fn batch_id(mut self, value: impl Into<String>) -> Self {
        self.batch_id = Some(value.into());
        self
    }

    pub fn bcc(mut self, value: Vec<SendTransacEmailRequestBccItem>) -> Self {
        self.bcc = Some(value);
        self
    }

    pub fn cc(mut self, value: Vec<SendTransacEmailRequestCcItem>) -> Self {
        self.cc = Some(value);
        self
    }

    pub fn headers(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn html_content(mut self, value: impl Into<String>) -> Self {
        self.html_content = Some(value.into());
        self
    }

    pub fn message_versions(
        mut self,
        value: Vec<SendTransacEmailRequestMessageVersionsItem>,
    ) -> Self {
        self.message_versions = Some(value);
        self
    }

    pub fn params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn reply_to(mut self, value: SendTransacEmailRequestReplyTo) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn scheduled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    pub fn sender(mut self, value: SendTransacEmailRequestSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn text_content(mut self, value: impl Into<String>) -> Self {
        self.text_content = Some(value.into());
        self
    }

    pub fn to(mut self, value: Vec<SendTransacEmailRequestToItem>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTransacEmailRequest`].
    pub fn build(self) -> Result<SendTransacEmailRequest, BuildError> {
        Ok(SendTransacEmailRequest {
            attachment: self.attachment,
            batch_id: self.batch_id,
            bcc: self.bcc,
            cc: self.cc,
            headers: self.headers,
            html_content: self.html_content,
            message_versions: self.message_versions,
            params: self.params,
            reply_to: self.reply_to,
            scheduled_at: self.scheduled_at,
            sender: self.sender,
            subject: self.subject,
            tags: self.tags,
            template_id: self.template_id,
            text_content: self.text_content,
            to: self.to,
        })
    }
}
