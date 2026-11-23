pub use crate::prelude::*;

/// Schedule configuration for tier downgrades. Required when downgradeStrategy is set to a schedule-based strategy.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTierGroupRequestDowngradeSchedule {
    /// Duration value for the schedule.
    #[serde(rename = "durationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i64>,
    /// Duration unit for the schedule.
    #[serde(rename = "durationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<CreateTierGroupRequestDowngradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(rename = "durationModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_modifier: Option<CreateTierGroupRequestDowngradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(rename = "scheduledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_date: Option<String>,
}

impl CreateTierGroupRequestDowngradeSchedule {
    pub fn builder() -> CreateTierGroupRequestDowngradeScheduleBuilder {
        <CreateTierGroupRequestDowngradeScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTierGroupRequestDowngradeScheduleBuilder {
    duration_value: Option<i64>,
    duration_unit: Option<CreateTierGroupRequestDowngradeScheduleDurationUnit>,
    duration_modifier: Option<CreateTierGroupRequestDowngradeScheduleDurationModifier>,
    scheduled_date: Option<String>,
}

impl CreateTierGroupRequestDowngradeScheduleBuilder {
    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn duration_unit(
        mut self,
        value: CreateTierGroupRequestDowngradeScheduleDurationUnit,
    ) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn duration_modifier(
        mut self,
        value: CreateTierGroupRequestDowngradeScheduleDurationModifier,
    ) -> Self {
        self.duration_modifier = Some(value);
        self
    }

    pub fn scheduled_date(mut self, value: impl Into<String>) -> Self {
        self.scheduled_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTierGroupRequestDowngradeSchedule`].
    pub fn build(self) -> Result<CreateTierGroupRequestDowngradeSchedule, BuildError> {
        Ok(CreateTierGroupRequestDowngradeSchedule {
            duration_value: self.duration_value,
            duration_unit: self.duration_unit,
            duration_modifier: self.duration_modifier,
            scheduled_date: self.scheduled_date,
        })
    }
}
