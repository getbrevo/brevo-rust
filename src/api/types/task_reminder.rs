pub use crate::prelude::*;

/// Task reminder date/time for a task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskReminder {
    /// Type of task reminder e.g email, push
    #[serde(default)]
    pub types: Vec<TaskReminderTypesItem>,
    /// Unit of time before reminder is to be sent
    pub unit: TaskReminderUnit,
    /// Value of time unit before reminder is to be sent
    #[serde(default)]
    pub value: i64,
}

impl TaskReminder {
    pub fn builder() -> TaskReminderBuilder {
        <TaskReminderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskReminderBuilder {
    types: Option<Vec<TaskReminderTypesItem>>,
    unit: Option<TaskReminderUnit>,
    value: Option<i64>,
}

impl TaskReminderBuilder {
    pub fn types(mut self, value: Vec<TaskReminderTypesItem>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn unit(mut self, value: TaskReminderUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskReminder`].
    /// This method will fail if any of the following fields are not set:
    /// - [`types`](TaskReminderBuilder::types)
    /// - [`unit`](TaskReminderBuilder::unit)
    /// - [`value`](TaskReminderBuilder::value)
    pub fn build(self) -> Result<TaskReminder, BuildError> {
        Ok(TaskReminder {
            types: self
                .types
                .ok_or_else(|| BuildError::missing_field("types"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
