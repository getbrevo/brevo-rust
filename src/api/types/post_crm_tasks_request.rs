pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCrmTasksRequest {
    /// To assign a task to a user you can use either the account email or ID.
    #[serde(rename = "assignToId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_to_id: Option<String>,
    /// Companies ids for companies a task is linked to
    #[serde(rename = "companiesIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub companies_ids: Option<Vec<String>>,
    /// Contact ids for contacts linked to this task
    #[serde(rename = "contactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts_ids: Option<Vec<i64>>,
    /// Task due date and time
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub date: DateTime<FixedOffset>,
    /// Deal ids for deals a task is linked to
    #[serde(rename = "dealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deals_ids: Option<Vec<String>>,
    /// Task marked as done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    /// Duration of task in milliseconds [1 minute = 60000 ms]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Name of task
    #[serde(default)]
    pub name: String,
    /// Notes added to a task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder: Option<TaskReminder>,
    /// Id for type of task e.g Call / Email / Meeting etc.
    #[serde(rename = "taskTypeId")]
    #[serde(default)]
    pub task_type_id: String,
}

impl PostCrmTasksRequest {
    pub fn builder() -> PostCrmTasksRequestBuilder {
        <PostCrmTasksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmTasksRequestBuilder {
    assign_to_id: Option<String>,
    companies_ids: Option<Vec<String>>,
    contacts_ids: Option<Vec<i64>>,
    date: Option<DateTime<FixedOffset>>,
    deals_ids: Option<Vec<String>>,
    done: Option<bool>,
    duration: Option<i64>,
    name: Option<String>,
    notes: Option<String>,
    reminder: Option<TaskReminder>,
    task_type_id: Option<String>,
}

impl PostCrmTasksRequestBuilder {
    pub fn assign_to_id(mut self, value: impl Into<String>) -> Self {
        self.assign_to_id = Some(value.into());
        self
    }

    pub fn companies_ids(mut self, value: Vec<String>) -> Self {
        self.companies_ids = Some(value);
        self
    }

    pub fn contacts_ids(mut self, value: Vec<i64>) -> Self {
        self.contacts_ids = Some(value);
        self
    }

    pub fn date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date = Some(value);
        self
    }

    pub fn deals_ids(mut self, value: Vec<String>) -> Self {
        self.deals_ids = Some(value);
        self
    }

    pub fn done(mut self, value: bool) -> Self {
        self.done = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn reminder(mut self, value: TaskReminder) -> Self {
        self.reminder = Some(value);
        self
    }

    pub fn task_type_id(mut self, value: impl Into<String>) -> Self {
        self.task_type_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCrmTasksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostCrmTasksRequestBuilder::date)
    /// - [`name`](PostCrmTasksRequestBuilder::name)
    /// - [`task_type_id`](PostCrmTasksRequestBuilder::task_type_id)
    pub fn build(self) -> Result<PostCrmTasksRequest, BuildError> {
        Ok(PostCrmTasksRequest {
            assign_to_id: self.assign_to_id,
            companies_ids: self.companies_ids,
            contacts_ids: self.contacts_ids,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            deals_ids: self.deals_ids,
            done: self.done,
            duration: self.duration,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            notes: self.notes,
            reminder: self.reminder,
            task_type_id: self
                .task_type_id
                .ok_or_else(|| BuildError::missing_field("task_type_id"))?,
        })
    }
}
