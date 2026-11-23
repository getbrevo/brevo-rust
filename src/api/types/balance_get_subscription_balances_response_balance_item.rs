pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSubscriptionBalancesResponseBalanceItem {
    /// Balance definition ID.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Aggregate balance value for this definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl GetSubscriptionBalancesResponseBalanceItem {
    pub fn builder() -> GetSubscriptionBalancesResponseBalanceItemBuilder {
        <GetSubscriptionBalancesResponseBalanceItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSubscriptionBalancesResponseBalanceItemBuilder {
    balance_definition_id: Option<String>,
    value: Option<f64>,
}

impl GetSubscriptionBalancesResponseBalanceItemBuilder {
    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSubscriptionBalancesResponseBalanceItem`].
    pub fn build(self) -> Result<GetSubscriptionBalancesResponseBalanceItem, BuildError> {
        Ok(GetSubscriptionBalancesResponseBalanceItem {
            balance_definition_id: self.balance_definition_id,
            value: self.value,
        })
    }
}
