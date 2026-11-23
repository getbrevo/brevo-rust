pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTierRequestTierRewardsItem {
    /// Reward unique identifier
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
}

impl UpdateTierRequestTierRewardsItem {
    pub fn builder() -> UpdateTierRequestTierRewardsItemBuilder {
        <UpdateTierRequestTierRewardsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTierRequestTierRewardsItemBuilder {
    reward_id: Option<String>,
}

impl UpdateTierRequestTierRewardsItemBuilder {
    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateTierRequestTierRewardsItem`].
    pub fn build(self) -> Result<UpdateTierRequestTierRewardsItem, BuildError> {
        Ok(UpdateTierRequestTierRewardsItem {
            reward_id: self.reward_id,
        })
    }
}
