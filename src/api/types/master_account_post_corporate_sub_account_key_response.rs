pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateSubAccountKeyResponse {
    /// API key
    #[serde(default)]
    pub key: String,
    /// Status of the API operation.
    #[serde(default)]
    pub status: String,
}

impl PostCorporateSubAccountKeyResponse {
    pub fn builder() -> PostCorporateSubAccountKeyResponseBuilder {
        <PostCorporateSubAccountKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateSubAccountKeyResponseBuilder {
    key: Option<String>,
    status: Option<String>,
}

impl PostCorporateSubAccountKeyResponseBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateSubAccountKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostCorporateSubAccountKeyResponseBuilder::key)
    /// - [`status`](PostCorporateSubAccountKeyResponseBuilder::status)
    pub fn build(self) -> Result<PostCorporateSubAccountKeyResponse, BuildError> {
        Ok(PostCorporateSubAccountKeyResponse {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
