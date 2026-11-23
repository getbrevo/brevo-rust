pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendTransacEmailRequestMessageVersionsItem {
    /// Array of BCC recipient objects. Each object contains an email address and an optional name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<SendTransacEmailRequestMessageVersionsItemBccItem>>,
    /// Array of CC recipient objects. Each object contains an email address and an optional name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<SendTransacEmailRequestMessageVersionsItemCcItem>>,
    /// HTML body content of the email. Required when `templateId` is not provided. Ignored when `templateId` is provided.
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// Key-value pairs for template variable substitution. Only applicable when the template uses the New Template Language format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    /// Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<SendTransacEmailRequestMessageVersionsItemReplyTo>,
    /// Email subject line for this message version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Plain text body content of the email. Ignored when `templateId` is provided.
    #[serde(rename = "textContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<String>,
    /// Array of recipient objects. Each object contains an email address and an optional display name.
    #[serde(default)]
    pub to: Vec<SendTransacEmailRequestMessageVersionsItemToItem>,
}

impl SendTransacEmailRequestMessageVersionsItem {
    pub fn builder() -> SendTransacEmailRequestMessageVersionsItemBuilder {
        <SendTransacEmailRequestMessageVersionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacEmailRequestMessageVersionsItemBuilder {
    bcc: Option<Vec<SendTransacEmailRequestMessageVersionsItemBccItem>>,
    cc: Option<Vec<SendTransacEmailRequestMessageVersionsItemCcItem>>,
    html_content: Option<String>,
    params: Option<HashMap<String, serde_json::Value>>,
    reply_to: Option<SendTransacEmailRequestMessageVersionsItemReplyTo>,
    subject: Option<String>,
    text_content: Option<String>,
    to: Option<Vec<SendTransacEmailRequestMessageVersionsItemToItem>>,
}

impl SendTransacEmailRequestMessageVersionsItemBuilder {
    pub fn bcc(mut self, value: Vec<SendTransacEmailRequestMessageVersionsItemBccItem>) -> Self {
        self.bcc = Some(value);
        self
    }

    pub fn cc(mut self, value: Vec<SendTransacEmailRequestMessageVersionsItemCcItem>) -> Self {
        self.cc = Some(value);
        self
    }

    pub fn html_content(mut self, value: impl Into<String>) -> Self {
        self.html_content = Some(value.into());
        self
    }

    pub fn params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn reply_to(mut self, value: SendTransacEmailRequestMessageVersionsItemReplyTo) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn text_content(mut self, value: impl Into<String>) -> Self {
        self.text_content = Some(value.into());
        self
    }

    pub fn to(mut self, value: Vec<SendTransacEmailRequestMessageVersionsItemToItem>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTransacEmailRequestMessageVersionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`to`](SendTransacEmailRequestMessageVersionsItemBuilder::to)
    pub fn build(self) -> Result<SendTransacEmailRequestMessageVersionsItem, BuildError> {
        Ok(SendTransacEmailRequestMessageVersionsItem {
            bcc: self.bcc,
            cc: self.cc,
            html_content: self.html_content,
            params: self.params,
            reply_to: self.reply_to,
            subject: self.subject,
            text_content: self.text_content,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
