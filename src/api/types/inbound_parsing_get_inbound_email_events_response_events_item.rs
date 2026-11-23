pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInboundEmailEventsResponseEventsItem {
    /// Date when email was received on SMTP relay
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub date: DateTime<FixedOffset>,
    /// Recipient’s email address
    #[serde(default)]
    pub recipient: String,
    /// Sender’s email address
    #[serde(default)]
    pub sender: String,
    /// UUID that can be used to fetch additional data
    #[serde(default)]
    pub uuid: String,
}

impl GetInboundEmailEventsResponseEventsItem {
    pub fn builder() -> GetInboundEmailEventsResponseEventsItemBuilder {
        <GetInboundEmailEventsResponseEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInboundEmailEventsResponseEventsItemBuilder {
    date: Option<DateTime<FixedOffset>>,
    recipient: Option<String>,
    sender: Option<String>,
    uuid: Option<String>,
}

impl GetInboundEmailEventsResponseEventsItemBuilder {
    pub fn date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date = Some(value);
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

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetInboundEmailEventsResponseEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](GetInboundEmailEventsResponseEventsItemBuilder::date)
    /// - [`recipient`](GetInboundEmailEventsResponseEventsItemBuilder::recipient)
    /// - [`sender`](GetInboundEmailEventsResponseEventsItemBuilder::sender)
    /// - [`uuid`](GetInboundEmailEventsResponseEventsItemBuilder::uuid)
    pub fn build(self) -> Result<GetInboundEmailEventsResponseEventsItem, BuildError> {
        Ok(GetInboundEmailEventsResponseEventsItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            recipient: self
                .recipient
                .ok_or_else(|| BuildError::missing_field("recipient"))?,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
