pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetContactBalancesResponseBalancesItem {
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl GetContactBalancesResponseBalancesItem {
    pub fn builder() -> GetContactBalancesResponseBalancesItemBuilder {
        <GetContactBalancesResponseBalancesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactBalancesResponseBalancesItemBuilder {
    contact_id: Option<i64>,
    loyalty_subscription_id: Option<String>,
    updated_at: Option<String>,
    value: Option<f64>,
}

impl GetContactBalancesResponseBalancesItemBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactBalancesResponseBalancesItem`].
    pub fn build(self) -> Result<GetContactBalancesResponseBalancesItem, BuildError> {
        Ok(GetContactBalancesResponseBalancesItem {
            contact_id: self.contact_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
            updated_at: self.updated_at,
            value: self.value,
        })
    }
}
