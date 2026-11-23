pub use crate::prelude::*;

/// Task Details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Task {
    /// Unique task id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Id for type of task e.g Call / Email / Meeting etc.
    #[serde(rename = "taskTypeId")]
    #[serde(default)]
    pub task_type_id: String,
    /// Name of task
    #[serde(default)]
    pub name: String,
    /// Companies ids for companies a task is linked to
    #[serde(rename = "companiesIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_ids: Option<Vec<String>>,
    /// Deal ids for deals a task is linked to
    #[serde(rename = "dealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deals_ids: Option<Vec<String>>,
    /// Contact ids for contacts linked to this task
    #[serde(rename = "contactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts_ids: Option<Vec<i64>>,
    /// Account id of the user assigned to this task
    #[serde(rename = "assignToId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_to_id: Option<String>,
    /// Task due date and time
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub date: DateTime<FixedOffset>,
    /// Notes added to a task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Whether the task is marked as done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    /// Task creation date/time
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Task last update date/time
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        <TaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskBuilder {
    id: Option<String>,
    task_type_id: Option<String>,
    name: Option<String>,
    companies_ids: Option<Vec<String>>,
    deals_ids: Option<Vec<String>>,
    contacts_ids: Option<Vec<i64>>,
    assign_to_id: Option<String>,
    date: Option<DateTime<FixedOffset>>,
    notes: Option<String>,
    done: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl TaskBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn task_type_id(mut self, value: impl Into<String>) -> Self {
        self.task_type_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn companies_ids(mut self, value: Vec<String>) -> Self {
        self.companies_ids = Some(value);
        self
    }

    pub fn deals_ids(mut self, value: Vec<String>) -> Self {
        self.deals_ids = Some(value);
        self
    }

    pub fn contacts_ids(mut self, value: Vec<i64>) -> Self {
        self.contacts_ids = Some(value);
        self
    }

    pub fn assign_to_id(mut self, value: impl Into<String>) -> Self {
        self.assign_to_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn done(mut self, value: bool) -> Self {
        self.done = Some(value);
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

    /// Consumes the builder and constructs a [`Task`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_type_id`](TaskBuilder::task_type_id)
    /// - [`name`](TaskBuilder::name)
    /// - [`date`](TaskBuilder::date)
    pub fn build(self) -> Result<Task, BuildError> {
        Ok(Task {
            id: self.id,
            task_type_id: self
                .task_type_id
                .ok_or_else(|| BuildError::missing_field("task_type_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            companies_ids: self.companies_ids,
            deals_ids: self.deals_ids,
            contacts_ids: self.contacts_ids,
            assign_to_id: self.assign_to_id,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            notes: self.notes,
            done: self.done,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
