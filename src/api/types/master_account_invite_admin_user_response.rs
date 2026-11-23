pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteAdminUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl InviteAdminUserResponse {
    pub fn builder() -> InviteAdminUserResponseBuilder {
        <InviteAdminUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteAdminUserResponseBuilder {
    id: Option<String>,
}

impl InviteAdminUserResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InviteAdminUserResponse`].
    pub fn build(self) -> Result<InviteAdminUserResponse, BuildError> {
        Ok(InviteAdminUserResponse { id: self.id })
    }
}
