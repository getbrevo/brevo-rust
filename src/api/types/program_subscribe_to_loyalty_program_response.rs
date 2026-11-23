pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscribeToLoyaltyProgramResponse {
    /// Unique identifier of the organization.
    #[serde(rename = "organizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<i64>,
    /// Unique identifier of the loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Version number of the subscription.
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i64>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Unique identifier of the subscription.
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Metadata associated with the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Timestamp when the subscription was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the subscription was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl SubscribeToLoyaltyProgramResponse {
    pub fn builder() -> SubscribeToLoyaltyProgramResponseBuilder {
        <SubscribeToLoyaltyProgramResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeToLoyaltyProgramResponseBuilder {
    organization_id: Option<i64>,
    loyalty_program_id: Option<String>,
    version_id: Option<i64>,
    contact_id: Option<i64>,
    loyalty_subscription_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl SubscribeToLoyaltyProgramResponseBuilder {
    pub fn organization_id(mut self, value: i64) -> Self {
        self.organization_id = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: i64) -> Self {
        self.version_id = Some(value);
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
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

    /// Consumes the builder and constructs a [`SubscribeToLoyaltyProgramResponse`].
    pub fn build(self) -> Result<SubscribeToLoyaltyProgramResponse, BuildError> {
        Ok(SubscribeToLoyaltyProgramResponse {
            organization_id: self.organization_id,
            loyalty_program_id: self.loyalty_program_id,
            version_id: self.version_id,
            contact_id: self.contact_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
            meta: self.meta,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
