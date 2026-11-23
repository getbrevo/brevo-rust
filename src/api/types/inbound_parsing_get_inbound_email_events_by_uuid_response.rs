pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInboundEmailEventsByUuidResponse {
    /// List of attachments of the email. This will be present only after the processing is done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<GetInboundEmailEventsByUuidResponseAttachmentsItem>>,
    /// Date when email was delivered successfully to the client’s webhook
    #[serde(rename = "deliveredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<DateTime<FixedOffset>>,
    /// List of events/logs that describe the lifecycle of the email on the Brevo platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<GetInboundEmailEventsByUuidResponseLogsItem>>,
    /// Value of the Message-ID header. This will be present only after the processing is done.
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Date when email was received on SMTP relay
    #[serde(rename = "receivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub received_at: Option<DateTime<FixedOffset>>,
    /// Recipient’s email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// Sender’s email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// Value of the Subject header. This will be present only after the processing is done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl GetInboundEmailEventsByUuidResponse {
    pub fn builder() -> GetInboundEmailEventsByUuidResponseBuilder {
        <GetInboundEmailEventsByUuidResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInboundEmailEventsByUuidResponseBuilder {
    attachments: Option<Vec<GetInboundEmailEventsByUuidResponseAttachmentsItem>>,
    delivered_at: Option<DateTime<FixedOffset>>,
    logs: Option<Vec<GetInboundEmailEventsByUuidResponseLogsItem>>,
    message_id: Option<String>,
    received_at: Option<DateTime<FixedOffset>>,
    recipient: Option<String>,
    sender: Option<String>,
    subject: Option<String>,
}

impl GetInboundEmailEventsByUuidResponseBuilder {
    pub fn attachments(
        mut self,
        value: Vec<GetInboundEmailEventsByUuidResponseAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn delivered_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.delivered_at = Some(value);
        self
    }

    pub fn logs(mut self, value: Vec<GetInboundEmailEventsByUuidResponseLogsItem>) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn received_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.received_at = Some(value);
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

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetInboundEmailEventsByUuidResponse`].
    pub fn build(self) -> Result<GetInboundEmailEventsByUuidResponse, BuildError> {
        Ok(GetInboundEmailEventsByUuidResponse {
            attachments: self.attachments,
            delivered_at: self.delivered_at,
            logs: self.logs,
            message_id: self.message_id,
            received_at: self.received_at,
            recipient: self.recipient,
            sender: self.sender,
            subject: self.subject,
        })
    }
}
