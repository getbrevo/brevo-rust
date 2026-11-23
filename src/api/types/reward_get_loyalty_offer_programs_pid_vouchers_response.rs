pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyOfferProgramsPidVouchersResponse {
    /// Contact id associated with the current reward
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// List of all the rewards for current contact
    #[serde(rename = "contactRewards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_rewards: Option<Vec<GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem>>,
    /// Count of the rewards associated with the current contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Loyalty Program Id for the contact
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Loyalty Subscription Id for the contact
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
}

impl GetLoyaltyOfferProgramsPidVouchersResponse {
    pub fn builder() -> GetLoyaltyOfferProgramsPidVouchersResponseBuilder {
        <GetLoyaltyOfferProgramsPidVouchersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidVouchersResponseBuilder {
    contact_id: Option<i64>,
    contact_rewards: Option<Vec<GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem>>,
    count: Option<i64>,
    loyalty_program_id: Option<String>,
    loyalty_subscription_id: Option<String>,
}

impl GetLoyaltyOfferProgramsPidVouchersResponseBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn contact_rewards(
        mut self,
        value: Vec<GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem>,
    ) -> Self {
        self.contact_rewards = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidVouchersResponse`].
    pub fn build(self) -> Result<GetLoyaltyOfferProgramsPidVouchersResponse, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidVouchersResponse {
            contact_id: self.contact_id,
            contact_rewards: self.contact_rewards,
            count: self.count,
            loyalty_program_id: self.loyalty_program_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
        })
    }
}
