pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetEmailEventReportResponseEventsItem {
    /// UTC date-time on which the event has been generated
    #[serde(default)]
    pub date: String,
    /// Email address which generates the event
    #[serde(default)]
    pub email: String,
    /// Event which occurred
    pub event: GetEmailEventReportResponseEventsItemEvent,
    /// Sender email from which the emails are sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// IP from which the user has opened the email or clicked on the link (only available if the event is opened or clicks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The link which is sent to the user (only available if the event is requests or opened or clicks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Message ID which generated the event
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    /// Reason of bounce (only available if the event is hardbounce or softbounce)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Subject of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Tag of the email which generated the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// ID of the template (only available if the email is template based)
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
}

impl GetEmailEventReportResponseEventsItem {
    pub fn builder() -> GetEmailEventReportResponseEventsItemBuilder {
        <GetEmailEventReportResponseEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailEventReportResponseEventsItemBuilder {
    date: Option<String>,
    email: Option<String>,
    event: Option<GetEmailEventReportResponseEventsItemEvent>,
    from: Option<String>,
    ip: Option<String>,
    link: Option<String>,
    message_id: Option<String>,
    reason: Option<String>,
    subject: Option<String>,
    tag: Option<String>,
    template_id: Option<i64>,
}

impl GetEmailEventReportResponseEventsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn event(mut self, value: GetEmailEventReportResponseEventsItemEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
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

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailEventReportResponseEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](GetEmailEventReportResponseEventsItemBuilder::date)
    /// - [`email`](GetEmailEventReportResponseEventsItemBuilder::email)
    /// - [`event`](GetEmailEventReportResponseEventsItemBuilder::event)
    /// - [`message_id`](GetEmailEventReportResponseEventsItemBuilder::message_id)
    pub fn build(self) -> Result<GetEmailEventReportResponseEventsItem, BuildError> {
        Ok(GetEmailEventReportResponseEventsItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            from: self.from,
            ip: self.ip,
            link: self.link,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            reason: self.reason,
            subject: self.subject,
            tag: self.tag,
            template_id: self.template_id,
        })
    }
}
