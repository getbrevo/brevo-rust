pub use crate::prelude::*;

/// Query parameters for deleteContactMembers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteContactMembersQueryRequest {
    /// Comma-separated list of member contact IDs to delete from the subscription.
    #[serde(rename = "memberContactIds")]
    #[serde(default)]
    pub member_contact_ids: String,
}

impl DeleteContactMembersQueryRequest {
    pub fn builder() -> DeleteContactMembersQueryRequestBuilder {
        <DeleteContactMembersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteContactMembersQueryRequestBuilder {
    member_contact_ids: Option<String>,
}

impl DeleteContactMembersQueryRequestBuilder {
    pub fn member_contact_ids(mut self, value: impl Into<String>) -> Self {
        self.member_contact_ids = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteContactMembersQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`member_contact_ids`](DeleteContactMembersQueryRequestBuilder::member_contact_ids)
    pub fn build(self) -> Result<DeleteContactMembersQueryRequest, BuildError> {
        Ok(DeleteContactMembersQueryRequest {
            member_contact_ids: self
                .member_contact_ids
                .ok_or_else(|| BuildError::missing_field("member_contact_ids"))?,
        })
    }
}
