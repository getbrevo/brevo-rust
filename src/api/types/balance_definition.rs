pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BalanceDefinition {
    /// Unique identifier for the balance definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the balance definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Short description of the balance definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional image reference URL.
    #[serde(rename = "imageRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Additional metadata for the balance definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Unit of balance measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<BalanceDefinitionUnit>,
    /// Minimum allowable balance.
    #[serde(rename = "minAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_amount: Option<f64>,
    /// Maximum allowable balance.
    #[serde(rename = "maxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_amount: Option<f64>,
    /// Maximum credit allowed per operation.
    #[serde(rename = "maxCreditAmountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_credit_amount_limit: Option<f64>,
    /// Maximum debit allowed per operation.
    #[serde(rename = "maxDebitAmountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_debit_amount_limit: Option<f64>,
    /// Partial enables partial credit of balance if maximum balance limit is reached. Strict enables rejection of a transaction if it will breach the max balance limit.
    #[serde(rename = "balanceOptionAmountOvertakingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_option_amount_overtaking_strategy:
        Option<BalanceDefinitionBalanceOptionAmountOvertakingStrategy>,
    /// Rounding strategy for credit transactions.
    #[serde(rename = "balanceOptionCreditRounding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_option_credit_rounding: Option<BalanceDefinitionBalanceOptionCreditRounding>,
    /// Rounding strategy for debit transactions.
    #[serde(rename = "balanceOptionDebitRounding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_option_debit_rounding: Option<BalanceDefinitionBalanceOptionDebitRounding>,
    /// Number of time units before the balance expires.
    #[serde(rename = "balanceAvailabilityDurationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_availability_duration_value: Option<i64>,
    /// Unit of time for the balance availability duration.
    #[serde(rename = "balanceAvailabilityDurationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_availability_duration_unit:
        Option<BalanceDefinitionBalanceAvailabilityDurationUnit>,
    /// Modifier for balance availability duration. startOfPeriod causes expiry at the start of the period, endOfPeriod at the end. noModification uses the exact duration from the credit date.
    #[serde(rename = "balanceAvailabilityDurationModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_availability_duration_modifier:
        Option<BalanceDefinitionBalanceAvailabilityDurationModifier>,
    /// Fixed expiration date in dd/mm format. The balance expires when this date next appears in the calendar. Only one of balanceExpirationDate or balance availability duration fields can be used.
    #[serde(rename = "balanceExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_expiration_date: Option<String>,
    /// Timestamp of balance definition creation.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp of the last update.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// Timestamp of balance definition deletion (nullable).
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
}

impl BalanceDefinition {
    pub fn builder() -> BalanceDefinitionBuilder {
        <BalanceDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BalanceDefinitionBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    image_ref: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    unit: Option<BalanceDefinitionUnit>,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
    max_credit_amount_limit: Option<f64>,
    max_debit_amount_limit: Option<f64>,
    balance_option_amount_overtaking_strategy:
        Option<BalanceDefinitionBalanceOptionAmountOvertakingStrategy>,
    balance_option_credit_rounding: Option<BalanceDefinitionBalanceOptionCreditRounding>,
    balance_option_debit_rounding: Option<BalanceDefinitionBalanceOptionDebitRounding>,
    balance_availability_duration_value: Option<i64>,
    balance_availability_duration_unit: Option<BalanceDefinitionBalanceAvailabilityDurationUnit>,
    balance_availability_duration_modifier:
        Option<BalanceDefinitionBalanceAvailabilityDurationModifier>,
    balance_expiration_date: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
}

impl BalanceDefinitionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn image_ref(mut self, value: impl Into<String>) -> Self {
        self.image_ref = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn unit(mut self, value: BalanceDefinitionUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn min_amount(mut self, value: f64) -> Self {
        self.min_amount = Some(value);
        self
    }

    pub fn max_amount(mut self, value: f64) -> Self {
        self.max_amount = Some(value);
        self
    }

    pub fn max_credit_amount_limit(mut self, value: f64) -> Self {
        self.max_credit_amount_limit = Some(value);
        self
    }

    pub fn max_debit_amount_limit(mut self, value: f64) -> Self {
        self.max_debit_amount_limit = Some(value);
        self
    }

    pub fn balance_option_amount_overtaking_strategy(
        mut self,
        value: BalanceDefinitionBalanceOptionAmountOvertakingStrategy,
    ) -> Self {
        self.balance_option_amount_overtaking_strategy = Some(value);
        self
    }

    pub fn balance_option_credit_rounding(
        mut self,
        value: BalanceDefinitionBalanceOptionCreditRounding,
    ) -> Self {
        self.balance_option_credit_rounding = Some(value);
        self
    }

    pub fn balance_option_debit_rounding(
        mut self,
        value: BalanceDefinitionBalanceOptionDebitRounding,
    ) -> Self {
        self.balance_option_debit_rounding = Some(value);
        self
    }

    pub fn balance_availability_duration_value(mut self, value: i64) -> Self {
        self.balance_availability_duration_value = Some(value);
        self
    }

    pub fn balance_availability_duration_unit(
        mut self,
        value: BalanceDefinitionBalanceAvailabilityDurationUnit,
    ) -> Self {
        self.balance_availability_duration_unit = Some(value);
        self
    }

    pub fn balance_availability_duration_modifier(
        mut self,
        value: BalanceDefinitionBalanceAvailabilityDurationModifier,
    ) -> Self {
        self.balance_availability_duration_modifier = Some(value);
        self
    }

    pub fn balance_expiration_date(mut self, value: impl Into<String>) -> Self {
        self.balance_expiration_date = Some(value.into());
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

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BalanceDefinition`].
    pub fn build(self) -> Result<BalanceDefinition, BuildError> {
        Ok(BalanceDefinition {
            id: self.id,
            name: self.name,
            description: self.description,
            image_ref: self.image_ref,
            meta: self.meta,
            unit: self.unit,
            min_amount: self.min_amount,
            max_amount: self.max_amount,
            max_credit_amount_limit: self.max_credit_amount_limit,
            max_debit_amount_limit: self.max_debit_amount_limit,
            balance_option_amount_overtaking_strategy: self
                .balance_option_amount_overtaking_strategy,
            balance_option_credit_rounding: self.balance_option_credit_rounding,
            balance_option_debit_rounding: self.balance_option_debit_rounding,
            balance_availability_duration_value: self.balance_availability_duration_value,
            balance_availability_duration_unit: self.balance_availability_duration_unit,
            balance_availability_duration_modifier: self.balance_availability_duration_modifier,
            balance_expiration_date: self.balance_expiration_date,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        })
    }
}
