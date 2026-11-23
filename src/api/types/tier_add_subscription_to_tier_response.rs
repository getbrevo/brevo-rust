pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddSubscriptionToTierResponse {
    /// Contact to which the tier is assigned
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Group Id to which the tier is associated
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Unique identifier for the assigned tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Associated loyalty program Id
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl AddSubscriptionToTierResponse {
    pub fn builder() -> AddSubscriptionToTierResponseBuilder {
        <AddSubscriptionToTierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddSubscriptionToTierResponseBuilder {
    contact_id: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    group_id: Option<String>,
    id: Option<String>,
    loyalty_program_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl AddSubscriptionToTierResponseBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddSubscriptionToTierResponse`].
    pub fn build(self) -> Result<AddSubscriptionToTierResponse, BuildError> {
        Ok(AddSubscriptionToTierResponse {
            contact_id: self.contact_id,
            created_at: self.created_at,
            group_id: self.group_id,
            id: self.id,
            loyalty_program_id: self.loyalty_program_id,
            meta: self.meta,
            updated_at: self.updated_at,
        })
    }
}
