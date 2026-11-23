pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateBalanceDefinitionRequest {
    /// Defines when the balance expires within the selected duration.
    #[serde(rename = "balanceAvailabilityDurationModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_availability_duration_modifier:
        Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier>,
    /// Unit of time for balance validity.
    #[serde(rename = "balanceAvailabilityDurationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_availability_duration_unit:
        Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit>,
    /// Number of time units before the balance expires.
    #[serde(rename = "balanceAvailabilityDurationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_availability_duration_value: Option<i64>,
    /// Expiration date (`dd/mm` format) or empty if not applicable.
    #[serde(rename = "balanceExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_expiration_date: Option<String>,
    /// Defines whether partial credit is allowed when reaching max balance.
    #[serde(rename = "balanceOptionAmountOvertakingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_option_amount_overtaking_strategy:
        Option<UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy>,
    /// Rounding strategy for credit transactions.
    #[serde(rename = "balanceOptionCreditRounding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_option_credit_rounding:
        Option<UpdateBalanceDefinitionRequestBalanceOptionCreditRounding>,
    /// Rounding strategy for debit transactions.
    #[serde(rename = "balanceOptionDebitRounding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_option_debit_rounding:
        Option<UpdateBalanceDefinitionRequestBalanceOptionDebitRounding>,
    /// Short description of the balance definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of an optional image reference.
    #[serde(rename = "imageRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Maximum allowable balance amount.
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
    /// Optional metadata for the balance definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<UpdateBalanceDefinitionRequestMeta>,
    /// Minimum allowable balance amount.
    #[serde(rename = "minAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_amount: Option<f64>,
    /// Name of the balance definition.
    #[serde(default)]
    pub name: String,
    /// Unit of balance measurement.
    pub unit: UpdateBalanceDefinitionRequestUnit,
}

impl UpdateBalanceDefinitionRequest {
    pub fn builder() -> UpdateBalanceDefinitionRequestBuilder {
        <UpdateBalanceDefinitionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBalanceDefinitionRequestBuilder {
    balance_availability_duration_modifier:
        Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier>,
    balance_availability_duration_unit:
        Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit>,
    balance_availability_duration_value: Option<i64>,
    balance_expiration_date: Option<String>,
    balance_option_amount_overtaking_strategy:
        Option<UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy>,
    balance_option_credit_rounding:
        Option<UpdateBalanceDefinitionRequestBalanceOptionCreditRounding>,
    balance_option_debit_rounding: Option<UpdateBalanceDefinitionRequestBalanceOptionDebitRounding>,
    description: Option<String>,
    image_ref: Option<String>,
    max_amount: Option<f64>,
    max_credit_amount_limit: Option<f64>,
    max_debit_amount_limit: Option<f64>,
    meta: Option<UpdateBalanceDefinitionRequestMeta>,
    min_amount: Option<f64>,
    name: Option<String>,
    unit: Option<UpdateBalanceDefinitionRequestUnit>,
}

impl UpdateBalanceDefinitionRequestBuilder {
    pub fn balance_availability_duration_modifier(
        mut self,
        value: UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier,
    ) -> Self {
        self.balance_availability_duration_modifier = Some(value);
        self
    }

    pub fn balance_availability_duration_unit(
        mut self,
        value: UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit,
    ) -> Self {
        self.balance_availability_duration_unit = Some(value);
        self
    }

    pub fn balance_availability_duration_value(mut self, value: i64) -> Self {
        self.balance_availability_duration_value = Some(value);
        self
    }

    pub fn balance_expiration_date(mut self, value: impl Into<String>) -> Self {
        self.balance_expiration_date = Some(value.into());
        self
    }

    pub fn balance_option_amount_overtaking_strategy(
        mut self,
        value: UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy,
    ) -> Self {
        self.balance_option_amount_overtaking_strategy = Some(value);
        self
    }

    pub fn balance_option_credit_rounding(
        mut self,
        value: UpdateBalanceDefinitionRequestBalanceOptionCreditRounding,
    ) -> Self {
        self.balance_option_credit_rounding = Some(value);
        self
    }

    pub fn balance_option_debit_rounding(
        mut self,
        value: UpdateBalanceDefinitionRequestBalanceOptionDebitRounding,
    ) -> Self {
        self.balance_option_debit_rounding = Some(value);
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

    pub fn meta(mut self, value: UpdateBalanceDefinitionRequestMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn min_amount(mut self, value: f64) -> Self {
        self.min_amount = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn unit(mut self, value: UpdateBalanceDefinitionRequestUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBalanceDefinitionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateBalanceDefinitionRequestBuilder::name)
    /// - [`unit`](UpdateBalanceDefinitionRequestBuilder::unit)
    pub fn build(self) -> Result<UpdateBalanceDefinitionRequest, BuildError> {
        Ok(UpdateBalanceDefinitionRequest {
            balance_availability_duration_modifier: self.balance_availability_duration_modifier,
            balance_availability_duration_unit: self.balance_availability_duration_unit,
            balance_availability_duration_value: self.balance_availability_duration_value,
            balance_expiration_date: self.balance_expiration_date,
            balance_option_amount_overtaking_strategy: self
                .balance_option_amount_overtaking_strategy,
            balance_option_credit_rounding: self.balance_option_credit_rounding,
            balance_option_debit_rounding: self.balance_option_debit_rounding,
            description: self.description,
            image_ref: self.image_ref,
            max_amount: self.max_amount,
            max_credit_amount_limit: self.max_credit_amount_limit,
            max_debit_amount_limit: self.max_debit_amount_limit,
            meta: self.meta,
            min_amount: self.min_amount,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
