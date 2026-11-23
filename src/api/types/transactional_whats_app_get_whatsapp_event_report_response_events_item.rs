pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetWhatsappEventReportResponseEventsItem {
    /// Text of the reply (will be there only in case of `reply` event with text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// WhatsApp Number with country code. Example, 85264318721
    #[serde(rename = "contactNumber")]
    #[serde(default)]
    pub contact_number: String,
    /// UTC date-time on which the event has been generated
    #[serde(default)]
    pub date: String,
    /// Event which occurred
    pub event: GetWhatsappEventReportResponseEventsItemEvent,
    /// Url of the media reply (will be there only in case of `reply` event with media)
    #[serde(rename = "mediaUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// Message ID which generated the event
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    /// Reason for the event (will be there in case of `error` and `soft-bounce` events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// WhatsApp Number with country code. Example, 85264318721
    #[serde(rename = "senderNumber")]
    #[serde(default)]
    pub sender_number: String,
}

impl GetWhatsappEventReportResponseEventsItem {
    pub fn builder() -> GetWhatsappEventReportResponseEventsItemBuilder {
        <GetWhatsappEventReportResponseEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsappEventReportResponseEventsItemBuilder {
    body: Option<String>,
    contact_number: Option<String>,
    date: Option<String>,
    event: Option<GetWhatsappEventReportResponseEventsItemEvent>,
    media_url: Option<String>,
    message_id: Option<String>,
    reason: Option<String>,
    sender_number: Option<String>,
}

impl GetWhatsappEventReportResponseEventsItemBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn contact_number(mut self, value: impl Into<String>) -> Self {
        self.contact_number = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn event(mut self, value: GetWhatsappEventReportResponseEventsItemEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn media_url(mut self, value: impl Into<String>) -> Self {
        self.media_url = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn sender_number(mut self, value: impl Into<String>) -> Self {
        self.sender_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsappEventReportResponseEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_number`](GetWhatsappEventReportResponseEventsItemBuilder::contact_number)
    /// - [`date`](GetWhatsappEventReportResponseEventsItemBuilder::date)
    /// - [`event`](GetWhatsappEventReportResponseEventsItemBuilder::event)
    /// - [`message_id`](GetWhatsappEventReportResponseEventsItemBuilder::message_id)
    /// - [`sender_number`](GetWhatsappEventReportResponseEventsItemBuilder::sender_number)
    pub fn build(self) -> Result<GetWhatsappEventReportResponseEventsItem, BuildError> {
        Ok(GetWhatsappEventReportResponseEventsItem {
            body: self.body,
            contact_number: self
                .contact_number
                .ok_or_else(|| BuildError::missing_field("contact_number"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            media_url: self.media_url,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            reason: self.reason,
            sender_number: self
                .sender_number
                .ok_or_else(|| BuildError::missing_field("sender_number"))?,
        })
    }
}
