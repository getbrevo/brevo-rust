pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTierForTierGroupRequest {
    #[serde(rename = "accessConditions")]
    #[serde(default)]
    pub access_conditions: Vec<CreateTierForTierGroupRequestAccessConditionsItem>,
    /// Image of the tier
    #[serde(rename = "imageRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Name of the tier to be created
    #[serde(default)]
    pub name: String,
    #[serde(rename = "tierRewards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_rewards: Option<Vec<CreateTierForTierGroupRequestTierRewardsItem>>,
}

impl CreateTierForTierGroupRequest {
    pub fn builder() -> CreateTierForTierGroupRequestBuilder {
        <CreateTierForTierGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTierForTierGroupRequestBuilder {
    access_conditions: Option<Vec<CreateTierForTierGroupRequestAccessConditionsItem>>,
    image_ref: Option<String>,
    name: Option<String>,
    tier_rewards: Option<Vec<CreateTierForTierGroupRequestTierRewardsItem>>,
}

impl CreateTierForTierGroupRequestBuilder {
    pub fn access_conditions(
        mut self,
        value: Vec<CreateTierForTierGroupRequestAccessConditionsItem>,
    ) -> Self {
        self.access_conditions = Some(value);
        self
    }

    pub fn image_ref(mut self, value: impl Into<String>) -> Self {
        self.image_ref = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tier_rewards(
        mut self,
        value: Vec<CreateTierForTierGroupRequestTierRewardsItem>,
    ) -> Self {
        self.tier_rewards = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTierForTierGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_conditions`](CreateTierForTierGroupRequestBuilder::access_conditions)
    /// - [`name`](CreateTierForTierGroupRequestBuilder::name)
    pub fn build(self) -> Result<CreateTierForTierGroupRequest, BuildError> {
        Ok(CreateTierForTierGroupRequest {
            access_conditions: self
                .access_conditions
                .ok_or_else(|| BuildError::missing_field("access_conditions"))?,
            image_ref: self.image_ref,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tier_rewards: self.tier_rewards,
        })
    }
}
