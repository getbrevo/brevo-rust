pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTierRequest {
    #[serde(rename = "accessConditions")]
    #[serde(default)]
    pub access_conditions: Vec<UpdateTierRequestAccessConditionsItem>,
    /// Image of the tier
    #[serde(rename = "imageRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Name of the tier to be created
    #[serde(default)]
    pub name: String,
    #[serde(rename = "tierRewards")]
    #[serde(default)]
    pub tier_rewards: Vec<UpdateTierRequestTierRewardsItem>,
}

impl UpdateTierRequest {
    pub fn builder() -> UpdateTierRequestBuilder {
        <UpdateTierRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTierRequestBuilder {
    access_conditions: Option<Vec<UpdateTierRequestAccessConditionsItem>>,
    image_ref: Option<String>,
    name: Option<String>,
    tier_rewards: Option<Vec<UpdateTierRequestTierRewardsItem>>,
}

impl UpdateTierRequestBuilder {
    pub fn access_conditions(mut self, value: Vec<UpdateTierRequestAccessConditionsItem>) -> Self {
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

    pub fn tier_rewards(mut self, value: Vec<UpdateTierRequestTierRewardsItem>) -> Self {
        self.tier_rewards = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTierRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_conditions`](UpdateTierRequestBuilder::access_conditions)
    /// - [`name`](UpdateTierRequestBuilder::name)
    /// - [`tier_rewards`](UpdateTierRequestBuilder::tier_rewards)
    pub fn build(self) -> Result<UpdateTierRequest, BuildError> {
        Ok(UpdateTierRequest {
            access_conditions: self
                .access_conditions
                .ok_or_else(|| BuildError::missing_field("access_conditions"))?,
            image_ref: self.image_ref,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tier_rewards: self
                .tier_rewards
                .ok_or_else(|| BuildError::missing_field("tier_rewards"))?,
        })
    }
}
