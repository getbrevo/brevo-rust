pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendTransacSmsResponse {
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: i64,
    #[serde(default)]
    pub reference: String,
    /// Remaining SMS credits of the user
    #[serde(rename = "remainingCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub remaining_credits: Option<f64>,
    /// Count of SMS's to send multiple text messages
    #[serde(rename = "smsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_count: Option<i64>,
    /// SMS credits used per text message
    #[serde(rename = "usedCredits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub used_credits: Option<f64>,
}

impl SendTransacSmsResponse {
    pub fn builder() -> SendTransacSmsResponseBuilder {
        <SendTransacSmsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacSmsResponseBuilder {
    message_id: Option<i64>,
    reference: Option<String>,
    remaining_credits: Option<f64>,
    sms_count: Option<i64>,
    used_credits: Option<f64>,
}

impl SendTransacSmsResponseBuilder {
    pub fn message_id(mut self, value: i64) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn remaining_credits(mut self, value: f64) -> Self {
        self.remaining_credits = Some(value);
        self
    }

    pub fn sms_count(mut self, value: i64) -> Self {
        self.sms_count = Some(value);
        self
    }

    pub fn used_credits(mut self, value: f64) -> Self {
        self.used_credits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTransacSmsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_id`](SendTransacSmsResponseBuilder::message_id)
    /// - [`reference`](SendTransacSmsResponseBuilder::reference)
    pub fn build(self) -> Result<SendTransacSmsResponse, BuildError> {
        Ok(SendTransacSmsResponse {
            message_id: self
                .message_id
                .ok_or_else(|| BuildError::missing_field("message_id"))?,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            remaining_credits: self.remaining_credits,
            sms_count: self.sms_count,
            used_credits: self.used_credits,
        })
    }
}
