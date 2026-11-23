pub use crate::prelude::*;

/// Created Note ID
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCrmNotesResponse {
    /// Unique note Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl PostCrmNotesResponse {
    pub fn builder() -> PostCrmNotesResponseBuilder {
        <PostCrmNotesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmNotesResponseBuilder {
    id: Option<String>,
}

impl PostCrmNotesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCrmNotesResponse`].
    pub fn build(self) -> Result<PostCrmNotesResponse, BuildError> {
        Ok(PostCrmNotesResponse { id: self.id })
    }
}
