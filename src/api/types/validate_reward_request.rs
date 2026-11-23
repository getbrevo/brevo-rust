pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidateRewardRequest {
    /// Unique identifier for the attributed reward
    #[serde(rename = "attributedRewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_reward_id: Option<String>,
    /// Validation code for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Unique identifier for the contact
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Identifier for the loyalty subscription
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Identifier for the point of sale
    #[serde(rename = "pointOfSellId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_of_sell_id: Option<String>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
}

impl ValidateRewardRequest {
    pub fn builder() -> ValidateRewardRequestBuilder {
        <ValidateRewardRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateRewardRequestBuilder {
    attributed_reward_id: Option<String>,
    code: Option<String>,
    contact_id: Option<i64>,
    loyalty_subscription_id: Option<String>,
    point_of_sell_id: Option<String>,
    reward_id: Option<String>,
}

impl ValidateRewardRequestBuilder {
    pub fn attributed_reward_id(mut self, value: impl Into<String>) -> Self {
        self.attributed_reward_id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn point_of_sell_id(mut self, value: impl Into<String>) -> Self {
        self.point_of_sell_id = Some(value.into());
        self
    }

    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidateRewardRequest`].
    pub fn build(self) -> Result<ValidateRewardRequest, BuildError> {
        Ok(ValidateRewardRequest {
            attributed_reward_id: self.attributed_reward_id,
            code: self.code,
            contact_id: self.contact_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
            point_of_sell_id: self.point_of_sell_id,
            reward_id: self.reward_id,
        })
    }
}
