pub use crate::prelude::*;

/// Created company id
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCompaniesResponse {
    /// Unique company id
    #[serde(default)]
    pub id: String,
}

impl PostCompaniesResponse {
    pub fn builder() -> PostCompaniesResponseBuilder {
        <PostCompaniesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCompaniesResponseBuilder {
    id: Option<String>,
}

impl PostCompaniesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCompaniesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCompaniesResponseBuilder::id)
    pub fn build(self) -> Result<PostCompaniesResponse, BuildError> {
        Ok(PostCompaniesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
