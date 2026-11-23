pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateListRequest {
    /// Id of the folder in which the list is to be moved. Either of the two parameters (name, folderId) can be updated at a time.
    #[serde(rename = "folderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    /// Name of the list. Either of the two parameters (name, folderId) can be updated at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateListRequest {
    pub fn builder() -> UpdateListRequestBuilder {
        <UpdateListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateListRequestBuilder {
    folder_id: Option<i64>,
    name: Option<String>,
}

impl UpdateListRequestBuilder {
    pub fn folder_id(mut self, value: i64) -> Self {
        self.folder_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateListRequest`].
    pub fn build(self) -> Result<UpdateListRequest, BuildError> {
        Ok(UpdateListRequest {
            folder_id: self.folder_id,
            name: self.name,
        })
    }
}
