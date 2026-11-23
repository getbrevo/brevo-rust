pub use crate::prelude::*;

/// Reason for blocking / unsubscribing
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacBlockedContactsResponseContactsItemReason {
    /// Reason code for blocking / unsubscribing (This code is safe for comparison)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<GetTransacBlockedContactsResponseContactsItemReasonCode>,
    /// Reason for blocking / unsubscribing (This string is not safe for comparison)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl GetTransacBlockedContactsResponseContactsItemReason {
    pub fn builder() -> GetTransacBlockedContactsResponseContactsItemReasonBuilder {
        <GetTransacBlockedContactsResponseContactsItemReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacBlockedContactsResponseContactsItemReasonBuilder {
    code: Option<GetTransacBlockedContactsResponseContactsItemReasonCode>,
    message: Option<String>,
}

impl GetTransacBlockedContactsResponseContactsItemReasonBuilder {
    pub fn code(mut self, value: GetTransacBlockedContactsResponseContactsItemReasonCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTransacBlockedContactsResponseContactsItemReason`].
    pub fn build(self) -> Result<GetTransacBlockedContactsResponseContactsItemReason, BuildError> {
        Ok(GetTransacBlockedContactsResponseContactsItemReason {
            code: self.code,
            message: self.message,
        })
    }
}
