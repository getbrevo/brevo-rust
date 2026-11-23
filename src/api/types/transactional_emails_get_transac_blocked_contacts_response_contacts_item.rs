pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacBlockedContactsResponseContactsItem {
    /// Date when the contact was blocked or unsubscribed on
    #[serde(rename = "blockedAt")]
    #[serde(default)]
    pub blocked_at: String,
    /// Email address of the blocked or unsubscribed contact
    #[serde(default)]
    pub email: String,
    /// Reason for blocking / unsubscribing
    #[serde(default)]
    pub reason: GetTransacBlockedContactsResponseContactsItemReason,
    /// Sender email address of the blocked or unsubscribed contact
    #[serde(rename = "senderEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
}

impl GetTransacBlockedContactsResponseContactsItem {
    pub fn builder() -> GetTransacBlockedContactsResponseContactsItemBuilder {
        <GetTransacBlockedContactsResponseContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacBlockedContactsResponseContactsItemBuilder {
    blocked_at: Option<String>,
    email: Option<String>,
    reason: Option<GetTransacBlockedContactsResponseContactsItemReason>,
    sender_email: Option<String>,
}

impl GetTransacBlockedContactsResponseContactsItemBuilder {
    pub fn blocked_at(mut self, value: impl Into<String>) -> Self {
        self.blocked_at = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn reason(mut self, value: GetTransacBlockedContactsResponseContactsItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn sender_email(mut self, value: impl Into<String>) -> Self {
        self.sender_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTransacBlockedContactsResponseContactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`blocked_at`](GetTransacBlockedContactsResponseContactsItemBuilder::blocked_at)
    /// - [`email`](GetTransacBlockedContactsResponseContactsItemBuilder::email)
    /// - [`reason`](GetTransacBlockedContactsResponseContactsItemBuilder::reason)
    pub fn build(self) -> Result<GetTransacBlockedContactsResponseContactsItem, BuildError> {
        Ok(GetTransacBlockedContactsResponseContactsItem {
            blocked_at: self
                .blocked_at
                .ok_or_else(|| BuildError::missing_field("blocked_at"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            sender_email: self.sender_email,
        })
    }
}
