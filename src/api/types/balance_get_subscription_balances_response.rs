pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSubscriptionBalancesResponse {
    /// Unique identifier of the loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Aggregate balance per balance definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<Vec<GetSubscriptionBalancesResponseBalanceItem>>,
}

impl GetSubscriptionBalancesResponse {
    pub fn builder() -> GetSubscriptionBalancesResponseBuilder {
        <GetSubscriptionBalancesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSubscriptionBalancesResponseBuilder {
    loyalty_program_id: Option<String>,
    contact_id: Option<i64>,
    balance: Option<Vec<GetSubscriptionBalancesResponseBalanceItem>>,
}

impl GetSubscriptionBalancesResponseBuilder {
    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn balance(mut self, value: Vec<GetSubscriptionBalancesResponseBalanceItem>) -> Self {
        self.balance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSubscriptionBalancesResponse`].
    pub fn build(self) -> Result<GetSubscriptionBalancesResponse, BuildError> {
        Ok(GetSubscriptionBalancesResponse {
            loyalty_program_id: self.loyalty_program_id,
            contact_id: self.contact_id,
            balance: self.balance,
        })
    }
}
