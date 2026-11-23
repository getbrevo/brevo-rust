pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetFolderListsResponse {
    /// Number of lists in the folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<GetList>>,
}

impl GetFolderListsResponse {
    pub fn builder() -> GetFolderListsResponseBuilder {
        <GetFolderListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetFolderListsResponseBuilder {
    count: Option<i64>,
    lists: Option<Vec<GetList>>,
}

impl GetFolderListsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn lists(mut self, value: Vec<GetList>) -> Self {
        self.lists = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetFolderListsResponse`].
    pub fn build(self) -> Result<GetFolderListsResponse, BuildError> {
        Ok(GetFolderListsResponse {
            count: self.count,
            lists: self.lists,
        })
    }
}
