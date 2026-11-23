pub use crate::prelude::*;

/// List of tasks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCrmTasksResponse {
    /// List of tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Task>>,
}

impl GetCrmTasksResponse {
    pub fn builder() -> GetCrmTasksResponseBuilder {
        <GetCrmTasksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrmTasksResponseBuilder {
    items: Option<Vec<Task>>,
}

impl GetCrmTasksResponseBuilder {
    pub fn items(mut self, value: Vec<Task>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCrmTasksResponse`].
    pub fn build(self) -> Result<GetCrmTasksResponse, BuildError> {
        Ok(GetCrmTasksResponse { items: self.items })
    }
}
