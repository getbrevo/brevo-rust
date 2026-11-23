pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTierForTierGroupRequestTierRewardsItem {
    /// Reward unique identifier
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
}

impl CreateTierForTierGroupRequestTierRewardsItem {
    pub fn builder() -> CreateTierForTierGroupRequestTierRewardsItemBuilder {
        <CreateTierForTierGroupRequestTierRewardsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTierForTierGroupRequestTierRewardsItemBuilder {
    reward_id: Option<String>,
}

impl CreateTierForTierGroupRequestTierRewardsItemBuilder {
    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTierForTierGroupRequestTierRewardsItem`].
    pub fn build(self) -> Result<CreateTierForTierGroupRequestTierRewardsItem, BuildError> {
        Ok(CreateTierForTierGroupRequestTierRewardsItem {
            reward_id: self.reward_id,
        })
    }
}
