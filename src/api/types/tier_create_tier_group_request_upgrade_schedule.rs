pub use crate::prelude::*;

/// Schedule configuration for tier upgrades. Required when upgradeStrategy is set to a schedule-based strategy.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTierGroupRequestUpgradeSchedule {
    /// Duration value for the schedule.
    #[serde(rename = "durationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i64>,
    /// Duration unit for the schedule.
    #[serde(rename = "durationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<CreateTierGroupRequestUpgradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(rename = "durationModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_modifier: Option<CreateTierGroupRequestUpgradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(rename = "scheduledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_date: Option<String>,
}

impl CreateTierGroupRequestUpgradeSchedule {
    pub fn builder() -> CreateTierGroupRequestUpgradeScheduleBuilder {
        <CreateTierGroupRequestUpgradeScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTierGroupRequestUpgradeScheduleBuilder {
    duration_value: Option<i64>,
    duration_unit: Option<CreateTierGroupRequestUpgradeScheduleDurationUnit>,
    duration_modifier: Option<CreateTierGroupRequestUpgradeScheduleDurationModifier>,
    scheduled_date: Option<String>,
}

impl CreateTierGroupRequestUpgradeScheduleBuilder {
    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn duration_unit(
        mut self,
        value: CreateTierGroupRequestUpgradeScheduleDurationUnit,
    ) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn duration_modifier(
        mut self,
        value: CreateTierGroupRequestUpgradeScheduleDurationModifier,
    ) -> Self {
        self.duration_modifier = Some(value);
        self
    }

    pub fn scheduled_date(mut self, value: impl Into<String>) -> Self {
        self.scheduled_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTierGroupRequestUpgradeSchedule`].
    pub fn build(self) -> Result<CreateTierGroupRequestUpgradeSchedule, BuildError> {
        Ok(CreateTierGroupRequestUpgradeSchedule {
            duration_value: self.duration_value,
            duration_unit: self.duration_unit,
            duration_modifier: self.duration_modifier,
            scheduled_date: self.scheduled_date,
        })
    }
}
