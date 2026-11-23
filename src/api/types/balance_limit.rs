pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BalanceLimit {
    /// Unique identifier for the balance limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Balance definition ID.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Specifies whether the limit applies to credit or debit transactions.
    #[serde(rename = "transactionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<BalanceLimitTransactionType>,
    /// Defines the type of constraint (transaction count or amount).
    #[serde(rename = "constraintType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_type: Option<BalanceLimitConstraintType>,
    /// Number of time units the balance limit applies to.
    #[serde(rename = "durationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i64>,
    /// Time unit for the balance limit.
    #[serde(rename = "durationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<BalanceLimitDurationUnit>,
    /// The maximum allowed value for the defined constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
    /// Indicates if the limit resets periodically based on a sliding schedule.
    #[serde(rename = "slidingSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sliding_schedule: Option<bool>,
    /// Timestamp of when the balance limit was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp of the last update to the balance limit.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl BalanceLimit {
    pub fn builder() -> BalanceLimitBuilder {
        <BalanceLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BalanceLimitBuilder {
    id: Option<String>,
    balance_definition_id: Option<String>,
    transaction_type: Option<BalanceLimitTransactionType>,
    constraint_type: Option<BalanceLimitConstraintType>,
    duration_value: Option<i64>,
    duration_unit: Option<BalanceLimitDurationUnit>,
    value: Option<f64>,
    sliding_schedule: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl BalanceLimitBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn transaction_type(mut self, value: BalanceLimitTransactionType) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn constraint_type(mut self, value: BalanceLimitConstraintType) -> Self {
        self.constraint_type = Some(value);
        self
    }

    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: BalanceLimitDurationUnit) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn sliding_schedule(mut self, value: bool) -> Self {
        self.sliding_schedule = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BalanceLimit`].
    pub fn build(self) -> Result<BalanceLimit, BuildError> {
        Ok(BalanceLimit {
            id: self.id,
            balance_definition_id: self.balance_definition_id,
            transaction_type: self.transaction_type,
            constraint_type: self.constraint_type,
            duration_value: self.duration_value,
            duration_unit: self.duration_unit,
            value: self.value,
            sliding_schedule: self.sliding_schedule,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
