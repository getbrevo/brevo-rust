pub use crate::prelude::*;

/// Downloadable file link
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCrmFilesIdResponse {
    /// A unique link to download the requested file.
    #[serde(rename = "fileUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
}

impl GetCrmFilesIdResponse {
    pub fn builder() -> GetCrmFilesIdResponseBuilder {
        <GetCrmFilesIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrmFilesIdResponseBuilder {
    file_url: Option<String>,
}

impl GetCrmFilesIdResponseBuilder {
    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCrmFilesIdResponse`].
    pub fn build(self) -> Result<GetCrmFilesIdResponse, BuildError> {
        Ok(GetCrmFilesIdResponse {
            file_url: self.file_url,
        })
    }
}
