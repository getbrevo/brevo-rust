pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetParameterSubscriptionInfoResponse {
    /// Balance details for the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<GetParameterSubscriptionInfoResponseBalance>,
    /// Name of the loyalty program.
    #[serde(rename = "loyaltyProgramName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_name: Option<String>,
    /// List of members associated with the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<GetParameterSubscriptionInfoResponseMembersItem>>,
    /// Membership details of the subscription. Returned when the subscription could be resolved from the provided `contactId` or `loyaltySubscriptionId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<GetParameterSubscriptionInfoResponseMembership>,
    /// List of rewards associated with the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<Vec<GetParameterSubscriptionInfoResponseRewardItem>>,
    /// List of tier assignments for the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<Vec<GetParameterSubscriptionInfoResponseTierItem>>,
}

impl GetParameterSubscriptionInfoResponse {
    pub fn builder() -> GetParameterSubscriptionInfoResponseBuilder {
        <GetParameterSubscriptionInfoResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoResponseBuilder {
    balance: Option<GetParameterSubscriptionInfoResponseBalance>,
    loyalty_program_name: Option<String>,
    members: Option<Vec<GetParameterSubscriptionInfoResponseMembersItem>>,
    membership: Option<GetParameterSubscriptionInfoResponseMembership>,
    reward: Option<Vec<GetParameterSubscriptionInfoResponseRewardItem>>,
    tier: Option<Vec<GetParameterSubscriptionInfoResponseTierItem>>,
}

impl GetParameterSubscriptionInfoResponseBuilder {
    pub fn balance(mut self, value: GetParameterSubscriptionInfoResponseBalance) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn loyalty_program_name(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_name = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<GetParameterSubscriptionInfoResponseMembersItem>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn membership(mut self, value: GetParameterSubscriptionInfoResponseMembership) -> Self {
        self.membership = Some(value);
        self
    }

    pub fn reward(mut self, value: Vec<GetParameterSubscriptionInfoResponseRewardItem>) -> Self {
        self.reward = Some(value);
        self
    }

    pub fn tier(mut self, value: Vec<GetParameterSubscriptionInfoResponseTierItem>) -> Self {
        self.tier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoResponse`].
    pub fn build(self) -> Result<GetParameterSubscriptionInfoResponse, BuildError> {
        Ok(GetParameterSubscriptionInfoResponse {
            balance: self.balance,
            loyalty_program_name: self.loyalty_program_name,
            members: self.members,
            membership: self.membership,
            reward: self.reward,
            tier: self.tier,
        })
    }
}
