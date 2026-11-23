pub use crate::prelude::*;

/// Schedule configuration for tier downgrades.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TierGroupDowngradeSchedule {
    /// Schedule strategy type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    /// Duration value for the schedule.
    #[serde(rename = "durationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i64>,
    /// Duration unit for the schedule.
    #[serde(rename = "durationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<TierGroupDowngradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(rename = "durationModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_modifier: Option<TierGroupDowngradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(rename = "scheduledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_date: Option<String>,
}

impl TierGroupDowngradeSchedule {
    pub fn builder() -> TierGroupDowngradeScheduleBuilder {
        <TierGroupDowngradeScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierGroupDowngradeScheduleBuilder {
    strategy: Option<String>,
    duration_value: Option<i64>,
    duration_unit: Option<TierGroupDowngradeScheduleDurationUnit>,
    duration_modifier: Option<TierGroupDowngradeScheduleDurationModifier>,
    scheduled_date: Option<String>,
}

impl TierGroupDowngradeScheduleBuilder {
    pub fn strategy(mut self, value: impl Into<String>) -> Self {
        self.strategy = Some(value.into());
        self
    }

    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: TierGroupDowngradeScheduleDurationUnit) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn duration_modifier(mut self, value: TierGroupDowngradeScheduleDurationModifier) -> Self {
        self.duration_modifier = Some(value);
        self
    }

    pub fn scheduled_date(mut self, value: impl Into<String>) -> Self {
        self.scheduled_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TierGroupDowngradeSchedule`].
    pub fn build(self) -> Result<TierGroupDowngradeSchedule, BuildError> {
        Ok(TierGroupDowngradeSchedule {
            strategy: self.strategy,
            duration_value: self.duration_value,
            duration_unit: self.duration_unit,
            duration_modifier: self.duration_modifier,
            scheduled_date: self.scheduled_date,
        })
    }
}
