pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSmsCampaignRequest {
    /// Content of the message. The **maximum characters used per SMS is 160**, if used more than that, it will be counted as more than one SMS.
    #[serde(default)]
    pub content: String,
    /// Name of the campaign
    #[serde(default)]
    pub name: String,
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    #[serde(rename = "organisationPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<CreateSmsCampaignRequestRecipients>,
    /// UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters**
    #[serde(default)]
    pub sender: String,
    /// Format of the message. It indicates whether the content should be treated as unicode or not.
    #[serde(rename = "unicodeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_enabled: Option<bool>,
    /// Instructions to unsubscribe from future communications. Recommended by U.S. carriers. Must include **STOP** keyword. This will be added as instructions after the end of message content. **Prefer verifying maximum length of 160 characters including this instructions in message content to avoid multiple sending of same sms.**
    #[serde(rename = "unsubscribeInstruction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_instruction: Option<String>,
}

impl CreateSmsCampaignRequest {
    pub fn builder() -> CreateSmsCampaignRequestBuilder {
        <CreateSmsCampaignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSmsCampaignRequestBuilder {
    content: Option<String>,
    name: Option<String>,
    organisation_prefix: Option<String>,
    recipients: Option<CreateSmsCampaignRequestRecipients>,
    scheduled_at: Option<String>,
    sender: Option<String>,
    unicode_enabled: Option<bool>,
    unsubscribe_instruction: Option<String>,
}

impl CreateSmsCampaignRequestBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organisation_prefix(mut self, value: impl Into<String>) -> Self {
        self.organisation_prefix = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: CreateSmsCampaignRequestRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn scheduled_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_at = Some(value.into());
        self
    }

    pub fn sender(mut self, value: impl Into<String>) -> Self {
        self.sender = Some(value.into());
        self
    }

    pub fn unicode_enabled(mut self, value: bool) -> Self {
        self.unicode_enabled = Some(value);
        self
    }

    pub fn unsubscribe_instruction(mut self, value: impl Into<String>) -> Self {
        self.unsubscribe_instruction = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSmsCampaignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](CreateSmsCampaignRequestBuilder::content)
    /// - [`name`](CreateSmsCampaignRequestBuilder::name)
    /// - [`sender`](CreateSmsCampaignRequestBuilder::sender)
    pub fn build(self) -> Result<CreateSmsCampaignRequest, BuildError> {
        Ok(CreateSmsCampaignRequest {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organisation_prefix: self.organisation_prefix,
            recipients: self.recipients,
            scheduled_at: self.scheduled_at,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            unicode_enabled: self.unicode_enabled,
            unsubscribe_instruction: self.unsubscribe_instruction,
        })
    }
}
