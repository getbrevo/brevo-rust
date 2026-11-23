pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem {
    /// Name of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Selected value of the parameter to define the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueResponse>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem {
    pub fn builder(
    ) -> GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItemBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItemBuilder {
    name: Option<String>,
    value: Option<ValueResponse>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: ValueResponse) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem, BuildError>
    {
        Ok(
            GetLoyaltyOfferProgramsPidRewardsRidResponseRuleResultsItemParametersItem {
                name: self.name,
                value: self.value,
            },
        )
    }
}
