pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendTransacSms {
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    #[serde(rename = "organisationPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation_prefix: Option<String>,
    /// Mobile number to send SMS with the country code. Must contain between 6 and 15 digits, optionally prefixed with '+'.
    #[serde(default)]
    pub recipient: String,
    /// Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters.** Alphanumeric sender names (up to 11 characters) must contain only letters and digits. Numeric sender names (12-15 characters) must contain only digits.
    #[serde(default)]
    pub sender: String,
    /// Tag of the message. Can be a single string or an array of strings (maximum 10 tags). Each tag must be a non-empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<SendTransacSmsTag>,
    /// Type of the SMS. Marketing SMS messages are those sent typically with marketing content. Transactional SMS messages are sent to individuals and are triggered in response to some action, such as a sign-up, purchase, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SendTransacSmsType>,
    /// Format of the message. It indicates whether the content should be treated as unicode or not.
    #[serde(rename = "unicodeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_enabled: Option<bool>,
    /// Webhook to call for each event triggered by the message (delivered etc.)
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    /// Pass the set of attributes to customize the template. For example, {"FNAME":"Joe", "LNAME":"Doe"}. These are the placeholder variables in the template that will be replaced with the corresponding values passed in the params object. Applicable only if `templateId` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    /// Template ID to send SMS with the template. When provided, overrides the content parameter. Mandatory if 'content' is not passed.
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// Content of the message. If more than **160 characters** long, will be sent as multiple text messages. Mandatory if 'templateId' is not passed, ignored if 'templateId' is passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl SendTransacSms {
    pub fn builder() -> SendTransacSmsBuilder {
        <SendTransacSmsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacSmsBuilder {
    organisation_prefix: Option<String>,
    recipient: Option<String>,
    sender: Option<String>,
    tag: Option<SendTransacSmsTag>,
    r#type: Option<SendTransacSmsType>,
    unicode_enabled: Option<bool>,
    web_url: Option<String>,
    params: Option<HashMap<String, serde_json::Value>>,
    template_id: Option<i64>,
    content: Option<String>,
}

impl SendTransacSmsBuilder {
    pub fn organisation_prefix(mut self, value: impl Into<String>) -> Self {
        self.organisation_prefix = Some(value.into());
        self
    }

    pub fn recipient(mut self, value: impl Into<String>) -> Self {
        self.recipient = Some(value.into());
        self
    }

    pub fn sender(mut self, value: impl Into<String>) -> Self {
        self.sender = Some(value.into());
        self
    }

    pub fn tag(mut self, value: SendTransacSmsTag) -> Self {
        self.tag = Some(value);
        self
    }

    pub fn r#type(mut self, value: SendTransacSmsType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unicode_enabled(mut self, value: bool) -> Self {
        self.unicode_enabled = Some(value);
        self
    }

    pub fn web_url(mut self, value: impl Into<String>) -> Self {
        self.web_url = Some(value.into());
        self
    }

    pub fn params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendTransacSms`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recipient`](SendTransacSmsBuilder::recipient)
    /// - [`sender`](SendTransacSmsBuilder::sender)
    pub fn build(self) -> Result<SendTransacSms, BuildError> {
        Ok(SendTransacSms {
            organisation_prefix: self.organisation_prefix,
            recipient: self
                .recipient
                .ok_or_else(|| BuildError::missing_field("recipient"))?,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            tag: self.tag,
            r#type: self.r#type,
            unicode_enabled: self.unicode_enabled,
            web_url: self.web_url,
            params: self.params,
            template_id: self.template_id,
            content: self.content,
        })
    }
}
