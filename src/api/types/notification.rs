pub use crate::prelude::*;

/// Optional. Use this object if you want to let Brevo send an email to the contact, with the payment request URL. If empty, no notifications (message and reminders) will be sent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Notification {
    /// Channel used to send the notifications.
    pub channel: String,
    /// Use this field if you want to give more context to your contact about the payment request.
    #[serde(default)]
    pub text: String,
}

impl Notification {
    pub fn builder() -> NotificationBuilder {
        <NotificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationBuilder {
    channel: Option<String>,
    text: Option<String>,
}

impl NotificationBuilder {
    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Notification`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel`](NotificationBuilder::channel)
    /// - [`text`](NotificationBuilder::text)
    pub fn build(self) -> Result<Notification, BuildError> {
        Ok(Notification {
            channel: self
                .channel
                .ok_or_else(|| BuildError::missing_field("channel"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
