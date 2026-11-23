pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateSubAccountResponse {
    /// ID of the sub-account created
    #[serde(default)]
    pub id: i64,
}

impl PostCorporateSubAccountResponse {
    pub fn builder() -> PostCorporateSubAccountResponseBuilder {
        <PostCorporateSubAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateSubAccountResponseBuilder {
    id: Option<i64>,
}

impl PostCorporateSubAccountResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateSubAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCorporateSubAccountResponseBuilder::id)
    pub fn build(self) -> Result<PostCorporateSubAccountResponse, BuildError> {
        Ok(PostCorporateSubAccountResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
