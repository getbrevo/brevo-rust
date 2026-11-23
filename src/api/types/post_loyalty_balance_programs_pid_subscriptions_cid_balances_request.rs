pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest {
    /// Unique identifier (UUID) of the balance definition associated with the new balance.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(default)]
    pub balance_definition_id: String,
}

impl PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest {
    pub fn builder() -> PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequestBuilder {
        <PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequestBuilder {
    balance_definition_id: Option<String>,
}

impl PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequestBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance_definition_id`](PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequestBuilder::balance_definition_id)
    pub fn build(
        self,
    ) -> Result<PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest, BuildError> {
        Ok(
            PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest {
                balance_definition_id: self
                    .balance_definition_id
                    .ok_or_else(|| BuildError::missing_field("balance_definition_id"))?,
            },
        )
    }
}
