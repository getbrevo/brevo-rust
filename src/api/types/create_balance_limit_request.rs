pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBalanceLimitRequest {
    /// Defines whether the limit applies to transaction count or amount.
    #[serde(rename = "constraintType")]
    pub constraint_type: CreateBalanceLimitRequestConstraintType,
    /// Unit of time for which the limit is applicable.
    #[serde(rename = "durationUnit")]
    pub duration_unit: CreateBalanceLimitRequestDurationUnit,
    /// Number of time units for the balance limit.
    #[serde(rename = "durationValue")]
    #[serde(default)]
    pub duration_value: i64,
    /// Determines if the limit resets on a rolling schedule.
    #[serde(rename = "slidingSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sliding_schedule: Option<bool>,
    /// Specifies whether the limit applies to credit or debit transactions.
    #[serde(rename = "transactionType")]
    pub transaction_type: CreateBalanceLimitRequestTransactionType,
    /// Maximum allowed value for the specified constraint type.
    #[serde(default)]
    pub value: i64,
}

impl CreateBalanceLimitRequest {
    pub fn builder() -> CreateBalanceLimitRequestBuilder {
        <CreateBalanceLimitRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBalanceLimitRequestBuilder {
    constraint_type: Option<CreateBalanceLimitRequestConstraintType>,
    duration_unit: Option<CreateBalanceLimitRequestDurationUnit>,
    duration_value: Option<i64>,
    sliding_schedule: Option<bool>,
    transaction_type: Option<CreateBalanceLimitRequestTransactionType>,
    value: Option<i64>,
}

impl CreateBalanceLimitRequestBuilder {
    pub fn constraint_type(mut self, value: CreateBalanceLimitRequestConstraintType) -> Self {
        self.constraint_type = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: CreateBalanceLimitRequestDurationUnit) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn sliding_schedule(mut self, value: bool) -> Self {
        self.sliding_schedule = Some(value);
        self
    }

    pub fn transaction_type(mut self, value: CreateBalanceLimitRequestTransactionType) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBalanceLimitRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`constraint_type`](CreateBalanceLimitRequestBuilder::constraint_type)
    /// - [`duration_unit`](CreateBalanceLimitRequestBuilder::duration_unit)
    /// - [`duration_value`](CreateBalanceLimitRequestBuilder::duration_value)
    /// - [`transaction_type`](CreateBalanceLimitRequestBuilder::transaction_type)
    /// - [`value`](CreateBalanceLimitRequestBuilder::value)
    pub fn build(self) -> Result<CreateBalanceLimitRequest, BuildError> {
        Ok(CreateBalanceLimitRequest {
            constraint_type: self
                .constraint_type
                .ok_or_else(|| BuildError::missing_field("constraint_type"))?,
            duration_unit: self
                .duration_unit
                .ok_or_else(|| BuildError::missing_field("duration_unit"))?,
            duration_value: self
                .duration_value
                .ok_or_else(|| BuildError::missing_field("duration_value"))?,
            sliding_schedule: self.sliding_schedule,
            transaction_type: self
                .transaction_type
                .ok_or_else(|| BuildError::missing_field("transaction_type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
