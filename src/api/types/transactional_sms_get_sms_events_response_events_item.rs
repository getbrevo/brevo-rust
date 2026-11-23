pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsEventsResponseEventsItem {
    /// UTC date-time on which the event has been generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Event which occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<GetSmsEventsResponseEventsItemEvent>,
    /// Message ID which generated the event
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Phone number which has generated the event
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Reason of bounce (only available if the event is hardbounce or softbounce)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<String>,
    /// Tag of the SMS which generated the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl GetSmsEventsResponseEventsItem {
    pub fn builder() -> GetSmsEventsResponseEventsItemBuilder {
        <GetSmsEventsResponseEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsEventsResponseEventsItemBuilder {
    date: Option<String>,
    event: Option<GetSmsEventsResponseEventsItemEvent>,
    message_id: Option<String>,
    phone_number: Option<String>,
    reason: Option<String>,
    reply: Option<String>,
    tag: Option<String>,
}

impl GetSmsEventsResponseEventsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn event(mut self, value: GetSmsEventsResponseEventsItemEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn reply(mut self, value: impl Into<String>) -> Self {
        self.reply = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSmsEventsResponseEventsItem`].
    pub fn build(self) -> Result<GetSmsEventsResponseEventsItem, BuildError> {
        Ok(GetSmsEventsResponseEventsItem {
            date: self.date,
            event: self.event,
            message_id: self.message_id,
            phone_number: self.phone_number,
            reason: self.reason,
            reply: self.reply,
            tag: self.tag,
        })
    }
}
