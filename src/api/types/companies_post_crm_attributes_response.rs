pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCrmAttributesResponse {
    /// Unique ID of the created attribute
    #[serde(default)]
    pub id: String,
}

impl PostCrmAttributesResponse {
    pub fn builder() -> PostCrmAttributesResponseBuilder {
        <PostCrmAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmAttributesResponseBuilder {
    id: Option<String>,
}

impl PostCrmAttributesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCrmAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCrmAttributesResponseBuilder::id)
    pub fn build(self) -> Result<PostCrmAttributesResponse, BuildError> {
        Ok(PostCrmAttributesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
