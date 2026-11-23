pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateSubAccountKeyRequest {
    /// Id of the sub-account organization
    #[serde(default)]
    pub id: i64,
    /// Name of the API key
    #[serde(default)]
    pub name: String,
}

impl PostCorporateSubAccountKeyRequest {
    pub fn builder() -> PostCorporateSubAccountKeyRequestBuilder {
        <PostCorporateSubAccountKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateSubAccountKeyRequestBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl PostCorporateSubAccountKeyRequestBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateSubAccountKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCorporateSubAccountKeyRequestBuilder::id)
    /// - [`name`](PostCorporateSubAccountKeyRequestBuilder::name)
    pub fn build(self) -> Result<PostCorporateSubAccountKeyRequest, BuildError> {
        Ok(PostCorporateSubAccountKeyRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
