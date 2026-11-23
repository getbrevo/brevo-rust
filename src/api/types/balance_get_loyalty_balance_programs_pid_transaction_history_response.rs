pub use crate::prelude::*;

/// Response containing transaction history details for a specific balance and contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyBalanceProgramsPidTransactionHistoryResponse {
    /// Unique identifier of the associated balance definition.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Unique identifier of the contact related to the transactions.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Total number of transactions in the history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Unique identifier of the associated loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// List of past transactions associated with the balance.
    #[serde(rename = "transactionHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_history:
        Option<Vec<GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem>>,
}

impl GetLoyaltyBalanceProgramsPidTransactionHistoryResponse {
    pub fn builder() -> GetLoyaltyBalanceProgramsPidTransactionHistoryResponseBuilder {
        <GetLoyaltyBalanceProgramsPidTransactionHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyBalanceProgramsPidTransactionHistoryResponseBuilder {
    balance_definition_id: Option<String>,
    contact_id: Option<i64>,
    count: Option<i64>,
    loyalty_program_id: Option<String>,
    transaction_history:
        Option<Vec<GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem>>,
}

impl GetLoyaltyBalanceProgramsPidTransactionHistoryResponseBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
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

    pub fn transaction_history(
        mut self,
        value: Vec<GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem>,
    ) -> Self {
        self.transaction_history = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyBalanceProgramsPidTransactionHistoryResponse`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyBalanceProgramsPidTransactionHistoryResponse, BuildError> {
        Ok(GetLoyaltyBalanceProgramsPidTransactionHistoryResponse {
            balance_definition_id: self.balance_definition_id,
            contact_id: self.contact_id,
            count: self.count,
            loyalty_program_id: self.loyalty_program_id,
            transaction_history: self.transaction_history,
        })
    }
}
