pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCompaniesImportResponse {
    /// The ID of the import process
    #[serde(rename = "processId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
}

impl PostCompaniesImportResponse {
    pub fn builder() -> PostCompaniesImportResponseBuilder {
        <PostCompaniesImportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCompaniesImportResponseBuilder {
    process_id: Option<i64>,
}

impl PostCompaniesImportResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCompaniesImportResponse`].
    pub fn build(self) -> Result<PostCompaniesImportResponse, BuildError> {
        Ok(PostCompaniesImportResponse {
            process_id: self.process_id,
        })
    }
}
