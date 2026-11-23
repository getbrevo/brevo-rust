pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateGroupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl PostCorporateGroupResponse {
    pub fn builder() -> PostCorporateGroupResponseBuilder {
        <PostCorporateGroupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateGroupResponseBuilder {
    id: Option<String>,
}

impl PostCorporateGroupResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateGroupResponse`].
    pub fn build(self) -> Result<PostCorporateGroupResponse, BuildError> {
        Ok(PostCorporateGroupResponse { id: self.id })
    }
}
