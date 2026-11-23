pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RedeemVoucherRequest {
    /// Unique identifier for the attributed reward
    #[serde(rename = "attributedRewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_reward_id: Option<String>,
    /// Redemption code for the reward
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
    /// Additional metadata associated with the redeem request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Order details for the redemption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<RedeemVoucherRequestOrder>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    /// Time to live in seconds for the redemption request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Whether the redemption should be automatically completed
    #[serde(rename = "autoComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_complete: Option<bool>,
}

impl RedeemVoucherRequest {
    pub fn builder() -> RedeemVoucherRequestBuilder {
        <RedeemVoucherRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemVoucherRequestBuilder {
    attributed_reward_id: Option<String>,
    code: Option<String>,
    contact_id: Option<i64>,
    loyalty_subscription_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    order: Option<RedeemVoucherRequestOrder>,
    reward_id: Option<String>,
    ttl: Option<i64>,
    auto_complete: Option<bool>,
}

impl RedeemVoucherRequestBuilder {
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

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn order(mut self, value: RedeemVoucherRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }

    pub fn auto_complete(mut self, value: bool) -> Self {
        self.auto_complete = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RedeemVoucherRequest`].
    pub fn build(self) -> Result<RedeemVoucherRequest, BuildError> {
        Ok(RedeemVoucherRequest {
            attributed_reward_id: self.attributed_reward_id,
            code: self.code,
            contact_id: self.contact_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
            meta: self.meta,
            order: self.order,
            reward_id: self.reward_id,
            ttl: self.ttl,
            auto_complete: self.auto_complete,
        })
    }
}
