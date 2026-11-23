pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EditUserPermissionResponse {
    /// Credit note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_notes: Option<Vec<String>>,
    /// Invoice id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Status of the API operation.
    #[serde(default)]
    pub status: String,
}

impl EditUserPermissionResponse {
    pub fn builder() -> EditUserPermissionResponseBuilder {
        <EditUserPermissionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditUserPermissionResponseBuilder {
    credit_notes: Option<Vec<String>>,
    invoice_id: Option<String>,
    status: Option<String>,
}

impl EditUserPermissionResponseBuilder {
    pub fn credit_notes(mut self, value: Vec<String>) -> Self {
        self.credit_notes = Some(value);
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditUserPermissionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](EditUserPermissionResponseBuilder::status)
    pub fn build(self) -> Result<EditUserPermissionResponse, BuildError> {
        Ok(EditUserPermissionResponse {
            credit_notes: self.credit_notes,
            invoice_id: self.invoice_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
