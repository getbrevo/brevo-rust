pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetParameterSubscriptionInfoResponseRewardItem {
    /// Reward code assigned to the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Timestamp when the reward was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Expiration date of the reward.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Unique identifier of the reward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique identifier of the loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Additional metadata related to the reward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Customer-facing description of the reward, as configured on the reward definition.
    #[serde(rename = "publicDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_description: Option<String>,
    /// Unique identifier of the reward definition.
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    /// Customer-facing name of the reward. Falls back to the internal reward name when no public name is set.
    #[serde(rename = "rewardName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_name: Option<String>,
    /// Unit the reward's value is expressed in — one of the currency codes listed below, or PERCENT. Omitted when the reward has no unit (e.g. free-product rewards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<GetParameterSubscriptionInfoResponseRewardItemUnit>,
    /// Timestamp when the reward was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// Date from which the voucher becomes valid.
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub valid_from: Option<DateTime<FixedOffset>>,
    /// The value recorded when this reward was attributed to the contact — a snapshot, not necessarily the reward's current configured value. Omitted when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl GetParameterSubscriptionInfoResponseRewardItem {
    pub fn builder() -> GetParameterSubscriptionInfoResponseRewardItemBuilder {
        <GetParameterSubscriptionInfoResponseRewardItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoResponseRewardItemBuilder {
    code: Option<String>,
    contact_id: Option<i64>,
    created_at: Option<String>,
    expiration_date: Option<String>,
    id: Option<String>,
    loyalty_program_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    public_description: Option<String>,
    reward_id: Option<String>,
    reward_name: Option<String>,
    unit: Option<GetParameterSubscriptionInfoResponseRewardItemUnit>,
    updated_at: Option<DateTime<FixedOffset>>,
    valid_from: Option<DateTime<FixedOffset>>,
    value: Option<f64>,
}

impl GetParameterSubscriptionInfoResponseRewardItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
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

    pub fn public_description(mut self, value: impl Into<String>) -> Self {
        self.public_description = Some(value.into());
        self
    }

    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    pub fn reward_name(mut self, value: impl Into<String>) -> Self {
        self.reward_name = Some(value.into());
        self
    }

    pub fn unit(mut self, value: GetParameterSubscriptionInfoResponseRewardItemUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn valid_from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.valid_from = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoResponseRewardItem`].
    pub fn build(self) -> Result<GetParameterSubscriptionInfoResponseRewardItem, BuildError> {
        Ok(GetParameterSubscriptionInfoResponseRewardItem {
            code: self.code,
            contact_id: self.contact_id,
            created_at: self.created_at,
            expiration_date: self.expiration_date,
            id: self.id,
            loyalty_program_id: self.loyalty_program_id,
            meta: self.meta,
            public_description: self.public_description,
            reward_id: self.reward_id,
            reward_name: self.reward_name,
            unit: self.unit,
            updated_at: self.updated_at,
            valid_from: self.valid_from,
            value: self.value,
        })
    }
}
