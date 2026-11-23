pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendWhatsappMessageRequestText {
    /// List of phone numbers of the contacts
    #[serde(rename = "contactNumbers")]
    #[serde(default)]
    pub contact_numbers: Vec<String>,
    /// WhatsApp Number with country code. Example, 85264318721
    #[serde(rename = "senderNumber")]
    #[serde(default)]
    pub sender_number: String,
    /// Text to be sent as message body (will be overridden if templateId is passed in the same request)
    #[serde(default)]
    pub text: String,
}

impl SendWhatsappMessageRequestText {
    pub fn builder() -> SendWhatsappMessageRequestTextBuilder {
        <SendWhatsappMessageRequestTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendWhatsappMessageRequestTextBuilder {
    contact_numbers: Option<Vec<String>>,
    sender_number: Option<String>,
    text: Option<String>,
}

impl SendWhatsappMessageRequestTextBuilder {
    pub fn contact_numbers(mut self, value: Vec<String>) -> Self {
        self.contact_numbers = Some(value);
        self
    }

    pub fn sender_number(mut self, value: impl Into<String>) -> Self {
        self.sender_number = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendWhatsappMessageRequestText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_numbers`](SendWhatsappMessageRequestTextBuilder::contact_numbers)
    /// - [`sender_number`](SendWhatsappMessageRequestTextBuilder::sender_number)
    /// - [`text`](SendWhatsappMessageRequestTextBuilder::text)
    pub fn build(self) -> Result<SendWhatsappMessageRequestText, BuildError> {
        Ok(SendWhatsappMessageRequestText {
            contact_numbers: self
                .contact_numbers
                .ok_or_else(|| BuildError::missing_field("contact_numbers"))?,
            sender_number: self
                .sender_number
                .ok_or_else(|| BuildError::missing_field("sender_number"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
