pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem {
    /// Action for the defined rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Parameters to define the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem>>,
    /// Selected service to define the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemBuilder {
    action: Option<String>,
    parameters:
        Option<Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem>>,
    service: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn parameters(
        mut self,
        value: Vec<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem>,
    ) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem, BuildError> {
        Ok(
            GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItem {
                action: self.action,
                parameters: self.parameters,
                service: self.service,
            },
        )
    }
}
