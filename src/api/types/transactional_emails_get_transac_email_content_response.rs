pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacEmailContentResponse {
    /// Count of the attachments that were sent in the email
    #[serde(rename = "attachmentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<i64>,
    /// Actual content of the transactional email that has been sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Date on which transactional email was sent
    #[serde(default)]
    pub date: String,
    /// Email address to which transactional email has been sent
    #[serde(default)]
    pub email: String,
    /// Series of events which occurred on the transactional email
    #[serde(default)]
    pub events: Vec<GetTransacEmailContentResponseEventsItem>,
    /// Subject of the sent email
    #[serde(default)]
    pub subject: String,
    /// Id of the template
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
}

impl GetTransacEmailContentResponse {
    pub fn builder() -> GetTransacEmailContentResponseBuilder {
        <GetTransacEmailContentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacEmailContentResponseBuilder {
    attachment_count: Option<i64>,
    body: Option<String>,
    date: Option<String>,
    email: Option<String>,
    events: Option<Vec<GetTransacEmailContentResponseEventsItem>>,
    subject: Option<String>,
    template_id: Option<i64>,
}

impl GetTransacEmailContentResponseBuilder {
    pub fn attachment_count(mut self, value: i64) -> Self {
        self.attachment_count = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<GetTransacEmailContentResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTransacEmailContentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](GetTransacEmailContentResponseBuilder::date)
    /// - [`email`](GetTransacEmailContentResponseBuilder::email)
    /// - [`events`](GetTransacEmailContentResponseBuilder::events)
    /// - [`subject`](GetTransacEmailContentResponseBuilder::subject)
    pub fn build(self) -> Result<GetTransacEmailContentResponse, BuildError> {
        Ok(GetTransacEmailContentResponse {
            attachment_count: self.attachment_count,
            body: self.body,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            template_id: self.template_id,
        })
    }
}
