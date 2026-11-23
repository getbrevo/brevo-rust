pub use crate::prelude::*;

/// Active balance response for a specific contact and balance definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyBalanceProgramsPidActiveBalanceResponse {
    /// Total number of active balances.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Unique identifier of the loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Unique identifier of the balance definition.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// List of active balances.
    #[serde(rename = "activeBalances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_balances:
        Option<Vec<GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem>>,
}

impl GetLoyaltyBalanceProgramsPidActiveBalanceResponse {
    pub fn builder() -> GetLoyaltyBalanceProgramsPidActiveBalanceResponseBuilder {
        <GetLoyaltyBalanceProgramsPidActiveBalanceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyBalanceProgramsPidActiveBalanceResponseBuilder {
    count: Option<i64>,
    loyalty_program_id: Option<String>,
    contact_id: Option<i64>,
    balance_definition_id: Option<String>,
    active_balances:
        Option<Vec<GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem>>,
}

impl GetLoyaltyBalanceProgramsPidActiveBalanceResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn active_balances(
        mut self,
        value: Vec<GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem>,
    ) -> Self {
        self.active_balances = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyBalanceProgramsPidActiveBalanceResponse`].
    pub fn build(self) -> Result<GetLoyaltyBalanceProgramsPidActiveBalanceResponse, BuildError> {
        Ok(GetLoyaltyBalanceProgramsPidActiveBalanceResponse {
            count: self.count,
            loyalty_program_id: self.loyalty_program_id,
            contact_id: self.contact_id,
            balance_definition_id: self.balance_definition_id,
            active_balances: self.active_balances,
        })
    }
}
