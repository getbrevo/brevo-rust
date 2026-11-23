pub use crate::prelude::*;

/// Schedule configuration for tier upgrades.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TierGroupUpgradeSchedule {
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
    pub duration_unit: Option<TierGroupUpgradeScheduleDurationUnit>,
    /// Modifier for the duration.
    #[serde(rename = "durationModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_modifier: Option<TierGroupUpgradeScheduleDurationModifier>,
    /// Scheduled date in DD/MM format.
    #[serde(rename = "scheduledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_date: Option<String>,
}

impl TierGroupUpgradeSchedule {
    pub fn builder() -> TierGroupUpgradeScheduleBuilder {
        <TierGroupUpgradeScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierGroupUpgradeScheduleBuilder {
    strategy: Option<String>,
    duration_value: Option<i64>,
    duration_unit: Option<TierGroupUpgradeScheduleDurationUnit>,
    duration_modifier: Option<TierGroupUpgradeScheduleDurationModifier>,
    scheduled_date: Option<String>,
}

impl TierGroupUpgradeScheduleBuilder {
    pub fn strategy(mut self, value: impl Into<String>) -> Self {
        self.strategy = Some(value.into());
        self
    }

    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: TierGroupUpgradeScheduleDurationUnit) -> Self {
        self.duration_unit = Some(value);
        self
    }

    pub fn duration_modifier(mut self, value: TierGroupUpgradeScheduleDurationModifier) -> Self {
        self.duration_modifier = Some(value);
        self
    }

    pub fn scheduled_date(mut self, value: impl Into<String>) -> Self {
        self.scheduled_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TierGroupUpgradeSchedule`].
    pub fn build(self) -> Result<TierGroupUpgradeSchedule, BuildError> {
        Ok(TierGroupUpgradeSchedule {
            strategy: self.strategy,
            duration_value: self.duration_value,
            duration_unit: self.duration_unit,
            duration_modifier: self.duration_modifier,
            scheduled_date: self.scheduled_date,
        })
    }
}
