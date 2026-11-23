pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs {
    /// Attribution config of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
    /// Code config of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Value config of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigsBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigsBuilder {
    attribution: Option<String>,
    code: Option<String>,
    value: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigsBuilder {
    pub fn attribution(mut self, value: impl Into<String>) -> Self {
        self.attribution = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidRewardsRidResponseRewardConfigs {
            attribution: self.attribution,
            code: self.code,
            value: self.value,
        })
    }
}
