pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetParameterSubscriptionInfoResponseBalanceBalancesItem {
    /// Unique identifier for the balance definition).
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Name of the balance definition.
    #[serde(rename = "balanceDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_name: Option<String>,
    /// The amount of the balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl GetParameterSubscriptionInfoResponseBalanceBalancesItem {
    pub fn builder() -> GetParameterSubscriptionInfoResponseBalanceBalancesItemBuilder {
        <GetParameterSubscriptionInfoResponseBalanceBalancesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoResponseBalanceBalancesItemBuilder {
    balance_definition_id: Option<String>,
    balance_definition_name: Option<String>,
    value: Option<f64>,
}

impl GetParameterSubscriptionInfoResponseBalanceBalancesItemBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn balance_definition_name(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_name = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoResponseBalanceBalancesItem`].
    pub fn build(
        self,
    ) -> Result<GetParameterSubscriptionInfoResponseBalanceBalancesItem, BuildError> {
        Ok(GetParameterSubscriptionInfoResponseBalanceBalancesItem {
            balance_definition_id: self.balance_definition_id,
            balance_definition_name: self.balance_definition_name,
            value: self.value,
        })
    }
}
