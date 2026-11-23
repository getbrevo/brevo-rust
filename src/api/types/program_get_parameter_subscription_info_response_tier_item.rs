pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetParameterSubscriptionInfoResponseTierItem {
    /// Unique identifier of the contact.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Timestamp when the tier was assigned.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Unique identifier of the group associated with the tier.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Name of the group associated with the tier.
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Unique identifier of the loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Additional metadata related to the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Unique identifier of the tier.
    #[serde(rename = "tierId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_id: Option<String>,
    /// Name of the tier.
    #[serde(rename = "tierName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_name: Option<String>,
    /// Timestamp when the tier was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl GetParameterSubscriptionInfoResponseTierItem {
    pub fn builder() -> GetParameterSubscriptionInfoResponseTierItemBuilder {
        <GetParameterSubscriptionInfoResponseTierItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoResponseTierItemBuilder {
    contact_id: Option<i64>,
    created_at: Option<String>,
    group_id: Option<String>,
    group_name: Option<String>,
    loyalty_program_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    tier_id: Option<String>,
    tier_name: Option<String>,
    updated_at: Option<String>,
}

impl GetParameterSubscriptionInfoResponseTierItemBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn tier_id(mut self, value: impl Into<String>) -> Self {
        self.tier_id = Some(value.into());
        self
    }

    pub fn tier_name(mut self, value: impl Into<String>) -> Self {
        self.tier_name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoResponseTierItem`].
    pub fn build(self) -> Result<GetParameterSubscriptionInfoResponseTierItem, BuildError> {
        Ok(GetParameterSubscriptionInfoResponseTierItem {
            contact_id: self.contact_id,
            created_at: self.created_at,
            group_id: self.group_id,
            group_name: self.group_name,
            loyalty_program_id: self.loyalty_program_id,
            meta: self.meta,
            tier_id: self.tier_id,
            tier_name: self.tier_name,
            updated_at: self.updated_at,
        })
    }
}
