pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent {
    /// Name of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEventBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEventBuilder {
    name: Option<String>,
    source: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEventBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidRewardsRidResponseRuleEvent {
            name: self.name,
            source: self.source,
        })
    }
}
