pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TierTierRewardsItem {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Reward to be attributed on tier assignment
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl TierTierRewardsItem {
    pub fn builder() -> TierTierRewardsItemBuilder {
        <TierTierRewardsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierTierRewardsItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    reward_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl TierTierRewardsItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TierTierRewardsItem`].
    pub fn build(self) -> Result<TierTierRewardsItem, BuildError> {
        Ok(TierTierRewardsItem {
            created_at: self.created_at,
            reward_id: self.reward_id,
            updated_at: self.updated_at,
        })
    }
}
