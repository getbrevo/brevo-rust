pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetFoldersResponse {
    /// Number of folders available in your account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<GetFolder>>,
}

impl GetFoldersResponse {
    pub fn builder() -> GetFoldersResponseBuilder {
        <GetFoldersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetFoldersResponseBuilder {
    count: Option<i64>,
    folders: Option<Vec<GetFolder>>,
}

impl GetFoldersResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn folders(mut self, value: Vec<GetFolder>) -> Self {
        self.folders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetFoldersResponse`].
    pub fn build(self) -> Result<GetFoldersResponse, BuildError> {
        Ok(GetFoldersResponse {
            count: self.count,
            folders: self.folders,
        })
    }
}
