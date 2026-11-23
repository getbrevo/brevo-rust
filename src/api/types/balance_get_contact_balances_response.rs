pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetContactBalancesResponse {
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<GetContactBalancesResponseBalancesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
}

impl GetContactBalancesResponse {
    pub fn builder() -> GetContactBalancesResponseBuilder {
        <GetContactBalancesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactBalancesResponseBuilder {
    balance_definition_id: Option<String>,
    balances: Option<Vec<GetContactBalancesResponseBalancesItem>>,
    count: Option<i64>,
    loyalty_program_id: Option<String>,
}

impl GetContactBalancesResponseBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn balances(mut self, value: Vec<GetContactBalancesResponseBalancesItem>) -> Self {
        self.balances = Some(value);
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

    /// Consumes the builder and constructs a [`GetContactBalancesResponse`].
    pub fn build(self) -> Result<GetContactBalancesResponse, BuildError> {
        Ok(GetContactBalancesResponse {
            balance_definition_id: self.balance_definition_id,
            balances: self.balances,
            count: self.count,
            loyalty_program_id: self.loyalty_program_id,
        })
    }
}
