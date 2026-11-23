pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacEmailsListResponseTransactionalEmailsItem {
    /// Date on which transactional email was sent
    #[serde(default)]
    pub date: String,
    /// Email address to which transactional email has been sent
    #[serde(default)]
    pub email: String,
    /// Email address of the sender from which the email was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Message Id of the sent email
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    /// Subject of the sent email
    #[serde(default)]
    pub subject: String,
    /// Tags used for your email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Id of the template
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// Unique id of the email sent to a particular contact
    #[serde(default)]
    pub uuid: String,
}

impl GetTransacEmailsListResponseTransactionalEmailsItem {
    pub fn builder() -> GetTransacEmailsListResponseTransactionalEmailsItemBuilder {
        <GetTransacEmailsListResponseTransactionalEmailsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacEmailsListResponseTransactionalEmailsItemBuilder {
    date: Option<String>,
    email: Option<String>,
    from: Option<String>,
    message_id: Option<String>,
    subject: Option<String>,
    tags: Option<Vec<String>>,
    template_id: Option<i64>,
    uuid: Option<String>,
}

impl GetTransacEmailsListResponseTransactionalEmailsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
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

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTransacEmailsListResponseTransactionalEmailsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](GetTransacEmailsListResponseTransactionalEmailsItemBuilder::date)
    /// - [`email`](GetTransacEmailsListResponseTransactionalEmailsItemBuilder::email)
    /// - [`message_id`](GetTransacEmailsListResponseTransactionalEmailsItemBuilder::message_id)
    /// - [`subject`](GetTransacEmailsListResponseTransactionalEmailsItemBuilder::subject)
    /// - [`uuid`](GetTransacEmailsListResponseTransactionalEmailsItemBuilder::uuid)
    pub fn build(self) -> Result<GetTransacEmailsListResponseTransactionalEmailsItem, BuildError> {
        Ok(GetTransacEmailsListResponseTransactionalEmailsItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            from: self.from,
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            tags: self.tags,
            template_id: self.template_id,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
