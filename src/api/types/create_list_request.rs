pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateListRequest {
    /// Id of the parent folder in which this list is to be created
    #[serde(rename = "folderId")]
    #[serde(default)]
    pub folder_id: i64,
    /// Name of the list
    #[serde(default)]
    pub name: String,
}

impl CreateListRequest {
    pub fn builder() -> CreateListRequestBuilder {
        <CreateListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateListRequestBuilder {
    folder_id: Option<i64>,
    name: Option<String>,
}

impl CreateListRequestBuilder {
    pub fn folder_id(mut self, value: i64) -> Self {
        self.folder_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`folder_id`](CreateListRequestBuilder::folder_id)
    /// - [`name`](CreateListRequestBuilder::name)
    pub fn build(self) -> Result<CreateListRequest, BuildError> {
        Ok(CreateListRequest {
            folder_id: self
                .folder_id
                .ok_or_else(|| BuildError::missing_field("folder_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
