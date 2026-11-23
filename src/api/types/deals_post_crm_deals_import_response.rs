pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCrmDealsImportResponse {
    /// The ID of the import process
    #[serde(rename = "processId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
}

impl PostCrmDealsImportResponse {
    pub fn builder() -> PostCrmDealsImportResponseBuilder {
        <PostCrmDealsImportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmDealsImportResponseBuilder {
    process_id: Option<i64>,
}

impl PostCrmDealsImportResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCrmDealsImportResponse`].
    pub fn build(self) -> Result<PostCrmDealsImportResponse, BuildError> {
        Ok(PostCrmDealsImportResponse {
            process_id: self.process_id,
        })
    }
}
