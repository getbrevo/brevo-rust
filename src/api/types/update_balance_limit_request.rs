pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateBalanceLimitRequest {
    /// Defines whether the limit applies to transaction count or amount.
    #[serde(rename = "constraintType")]
    pub constraint_type: UpdateBalanceLimitRequestConstraintType,
    /// Unit of time for which the limit is applicable.
    #[serde(rename = "durationUnit")]
    pub duration_unit: UpdateBalanceLimitRequestDurationUnit,
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
    pub transaction_type: UpdateBalanceLimitRequestTransactionType,
    /// Maximum allowed value for the specified constraint type.
    #[serde(default)]
    pub value: i64,
}

impl UpdateBalanceLimitRequest {
    pub fn builder() -> UpdateBalanceLimitRequestBuilder {
        <UpdateBalanceLimitRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBalanceLimitRequestBuilder {
    constraint_type: Option<UpdateBalanceLimitRequestConstraintType>,
    duration_unit: Option<UpdateBalanceLimitRequestDurationUnit>,
    duration_value: Option<i64>,
    sliding_schedule: Option<bool>,
    transaction_type: Option<UpdateBalanceLimitRequestTransactionType>,
    value: Option<i64>,
}

impl UpdateBalanceLimitRequestBuilder {
    pub fn constraint_type(mut self, value: UpdateBalanceLimitRequestConstraintType) -> Self {
        self.constraint_type = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: UpdateBalanceLimitRequestDurationUnit) -> Self {
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

    pub fn transaction_type(mut self, value: UpdateBalanceLimitRequestTransactionType) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBalanceLimitRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`constraint_type`](UpdateBalanceLimitRequestBuilder::constraint_type)
    /// - [`duration_unit`](UpdateBalanceLimitRequestBuilder::duration_unit)
    /// - [`duration_value`](UpdateBalanceLimitRequestBuilder::duration_value)
    /// - [`transaction_type`](UpdateBalanceLimitRequestBuilder::transaction_type)
    /// - [`value`](UpdateBalanceLimitRequestBuilder::value)
    pub fn build(self) -> Result<UpdateBalanceLimitRequest, BuildError> {
        Ok(UpdateBalanceLimitRequest {
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
