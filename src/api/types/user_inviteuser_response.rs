pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteuserResponse {
    /// Invoice id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Status of the API operation.
    #[serde(default)]
    pub status: String,
}

impl InviteuserResponse {
    pub fn builder() -> InviteuserResponseBuilder {
        <InviteuserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteuserResponseBuilder {
    invoice_id: Option<String>,
    status: Option<String>,
}

impl InviteuserResponseBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InviteuserResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](InviteuserResponseBuilder::status)
    pub fn build(self) -> Result<InviteuserResponse, BuildError> {
        Ok(InviteuserResponse {
            invoice_id: self.invoice_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
