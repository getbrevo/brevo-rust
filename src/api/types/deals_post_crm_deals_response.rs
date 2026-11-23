pub use crate::prelude::*;

/// Created deal id
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCrmDealsResponse {
    /// Unique deal id
    #[serde(default)]
    pub id: String,
}

impl PostCrmDealsResponse {
    pub fn builder() -> PostCrmDealsResponseBuilder {
        <PostCrmDealsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmDealsResponseBuilder {
    id: Option<String>,
}

impl PostCrmDealsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCrmDealsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCrmDealsResponseBuilder::id)
    pub fn build(self) -> Result<PostCrmDealsResponse, BuildError> {
        Ok(PostCrmDealsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
