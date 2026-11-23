pub use crate::prelude::*;

/// Task Details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCrmTasksResponse {
    /// Unique task id
    #[serde(default)]
    pub id: String,
}

impl PostCrmTasksResponse {
    pub fn builder() -> PostCrmTasksResponseBuilder {
        <PostCrmTasksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmTasksResponseBuilder {
    id: Option<String>,
}

impl PostCrmTasksResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCrmTasksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCrmTasksResponseBuilder::id)
    pub fn build(self) -> Result<PostCrmTasksResponse, BuildError> {
        Ok(PostCrmTasksResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
