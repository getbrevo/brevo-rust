pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Tier {
    /// Conditions required to access this tier
    #[serde(rename = "accessConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_conditions: Option<Vec<TierAccessConditionsItem>>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Associated group Id
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Tier image reference
    #[serde(rename = "imageRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Associated loyalty program Id
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Tier name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tier id
    #[serde(rename = "tierId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_id: Option<String>,
    /// Rewards associated with this tier
    #[serde(rename = "tierRewards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_rewards: Option<Vec<TierTierRewardsItem>>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl Tier {
    pub fn builder() -> TierBuilder {
        <TierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierBuilder {
    access_conditions: Option<Vec<TierAccessConditionsItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    group_id: Option<String>,
    image_ref: Option<String>,
    loyalty_program_id: Option<String>,
    name: Option<String>,
    tier_id: Option<String>,
    tier_rewards: Option<Vec<TierTierRewardsItem>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl TierBuilder {
    pub fn access_conditions(mut self, value: Vec<TierAccessConditionsItem>) -> Self {
        self.access_conditions = Some(value);
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

    pub fn image_ref(mut self, value: impl Into<String>) -> Self {
        self.image_ref = Some(value.into());
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tier_id(mut self, value: impl Into<String>) -> Self {
        self.tier_id = Some(value.into());
        self
    }

    pub fn tier_rewards(mut self, value: Vec<TierTierRewardsItem>) -> Self {
        self.tier_rewards = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Tier`].
    pub fn build(self) -> Result<Tier, BuildError> {
        Ok(Tier {
            access_conditions: self.access_conditions,
            created_at: self.created_at,
            group_id: self.group_id,
            image_ref: self.image_ref,
            loyalty_program_id: self.loyalty_program_id,
            name: self.name,
            tier_id: self.tier_id,
            tier_rewards: self.tier_rewards,
            updated_at: self.updated_at,
        })
    }
}
