pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem {
    /// Unique identifier of the balance entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Current balance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Timestamp when the balance was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the balance expires (if applicable).
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem {
    pub fn builder() -> GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItemBuilder {
        <GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItemBuilder {
    id: Option<String>,
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem, BuildError>
    {
        Ok(
            GetLoyaltyBalanceProgramsPidActiveBalanceResponseActiveBalancesItem {
                id: self.id,
                amount: self.amount,
                created_at: self.created_at,
                expires_at: self.expires_at,
            },
        )
    }
}
