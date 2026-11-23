pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutresendcancelinvitationResponse {
    /// Credit note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_notes: Option<Vec<String>>,
    /// Status of the API operation.
    #[serde(default)]
    pub status: String,
}

impl PutresendcancelinvitationResponse {
    pub fn builder() -> PutresendcancelinvitationResponseBuilder {
        <PutresendcancelinvitationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutresendcancelinvitationResponseBuilder {
    credit_notes: Option<Vec<String>>,
    status: Option<String>,
}

impl PutresendcancelinvitationResponseBuilder {
    pub fn credit_notes(mut self, value: Vec<String>) -> Self {
        self.credit_notes = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PutresendcancelinvitationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PutresendcancelinvitationResponseBuilder::status)
    pub fn build(self) -> Result<PutresendcancelinvitationResponse, BuildError> {
        Ok(PutresendcancelinvitationResponse {
            credit_notes: self.credit_notes,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
