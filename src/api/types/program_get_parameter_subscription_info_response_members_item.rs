pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetParameterSubscriptionInfoResponseMembersItem {
    /// Timestamp when the member was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Unique identifier of the member.
    #[serde(rename = "memberContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_contact_id: Option<i64>,
    /// Timestamp when the member was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl GetParameterSubscriptionInfoResponseMembersItem {
    pub fn builder() -> GetParameterSubscriptionInfoResponseMembersItemBuilder {
        <GetParameterSubscriptionInfoResponseMembersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoResponseMembersItemBuilder {
    created_at: Option<String>,
    member_contact_id: Option<i64>,
    updated_at: Option<String>,
}

impl GetParameterSubscriptionInfoResponseMembersItemBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn member_contact_id(mut self, value: i64) -> Self {
        self.member_contact_id = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoResponseMembersItem`].
    pub fn build(self) -> Result<GetParameterSubscriptionInfoResponseMembersItem, BuildError> {
        Ok(GetParameterSubscriptionInfoResponseMembersItem {
            created_at: self.created_at,
            member_contact_id: self.member_contact_id,
            updated_at: self.updated_at,
        })
    }
}
