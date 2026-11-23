pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutRevokeUserPermissionResponse {
    /// Credit note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_notes: Option<Vec<String>>,
    /// Status of the API operation.
    #[serde(default)]
    pub status: String,
}

impl PutRevokeUserPermissionResponse {
    pub fn builder() -> PutRevokeUserPermissionResponseBuilder {
        <PutRevokeUserPermissionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutRevokeUserPermissionResponseBuilder {
    credit_notes: Option<Vec<String>>,
    status: Option<String>,
}

impl PutRevokeUserPermissionResponseBuilder {
    pub fn credit_notes(mut self, value: Vec<String>) -> Self {
        self.credit_notes = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PutRevokeUserPermissionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PutRevokeUserPermissionResponseBuilder::status)
    pub fn build(self) -> Result<PutRevokeUserPermissionResponse, BuildError> {
        Ok(PutRevokeUserPermissionResponse {
            credit_notes: self.credit_notes,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
