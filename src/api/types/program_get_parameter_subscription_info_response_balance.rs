pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetParameterSubscriptionInfoResponseBalance {
    /// List of balance details associated with the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<GetParameterSubscriptionInfoResponseBalanceBalancesItem>>,
    /// Unique identifier of the contact.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Unique identifier of the loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
}

impl GetParameterSubscriptionInfoResponseBalance {
    pub fn builder() -> GetParameterSubscriptionInfoResponseBalanceBuilder {
        <GetParameterSubscriptionInfoResponseBalanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoResponseBalanceBuilder {
    balances: Option<Vec<GetParameterSubscriptionInfoResponseBalanceBalancesItem>>,
    contact_id: Option<i64>,
    loyalty_program_id: Option<String>,
}

impl GetParameterSubscriptionInfoResponseBalanceBuilder {
    pub fn balances(
        mut self,
        value: Vec<GetParameterSubscriptionInfoResponseBalanceBalancesItem>,
    ) -> Self {
        self.balances = Some(value);
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoResponseBalance`].
    pub fn build(self) -> Result<GetParameterSubscriptionInfoResponseBalance, BuildError> {
        Ok(GetParameterSubscriptionInfoResponseBalance {
            balances: self.balances,
            contact_id: self.contact_id,
            loyalty_program_id: self.loyalty_program_id,
        })
    }
}
