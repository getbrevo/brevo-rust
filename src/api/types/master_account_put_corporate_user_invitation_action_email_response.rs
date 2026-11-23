pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateUserInvitationActionEmailResponse {
    /// Action success message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl PutCorporateUserInvitationActionEmailResponse {
    pub fn builder() -> PutCorporateUserInvitationActionEmailResponseBuilder {
        <PutCorporateUserInvitationActionEmailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateUserInvitationActionEmailResponseBuilder {
    message: Option<String>,
}

impl PutCorporateUserInvitationActionEmailResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateUserInvitationActionEmailResponse`].
    pub fn build(self) -> Result<PutCorporateUserInvitationActionEmailResponse, BuildError> {
        Ok(PutCorporateUserInvitationActionEmailResponse {
            message: self.message,
        })
    }
}
