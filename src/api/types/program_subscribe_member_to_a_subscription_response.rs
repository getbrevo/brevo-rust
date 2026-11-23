pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscribeMemberToASubscriptionResponse {
    /// Unique identifier of the organization.
    #[serde(rename = "organizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<i64>,
    /// Unique identifier of the subscription owner.
    #[serde(rename = "ownerContactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact_id: Option<i64>,
    /// List of unique member contact IDs.
    #[serde(rename = "memberContactIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_contact_ids: Option<Vec<i64>>,
    /// Timestamp when the subscription member was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the subscription member was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl SubscribeMemberToASubscriptionResponse {
    pub fn builder() -> SubscribeMemberToASubscriptionResponseBuilder {
        <SubscribeMemberToASubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeMemberToASubscriptionResponseBuilder {
    organization_id: Option<i64>,
    owner_contact_id: Option<i64>,
    member_contact_ids: Option<Vec<i64>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl SubscribeMemberToASubscriptionResponseBuilder {
    pub fn organization_id(mut self, value: i64) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn owner_contact_id(mut self, value: i64) -> Self {
        self.owner_contact_id = Some(value);
        self
    }

    pub fn member_contact_ids(mut self, value: Vec<i64>) -> Self {
        self.member_contact_ids = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeMemberToASubscriptionResponse`].
    pub fn build(self) -> Result<SubscribeMemberToASubscriptionResponse, BuildError> {
        Ok(SubscribeMemberToASubscriptionResponse {
            organization_id: self.organization_id,
            owner_contact_id: self.owner_contact_id,
            member_contact_ids: self.member_contact_ids,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
