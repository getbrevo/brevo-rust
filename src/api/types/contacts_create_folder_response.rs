pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFolderResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateFolderResponse {
    pub fn builder() -> CreateFolderResponseBuilder {
        <CreateFolderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFolderResponseBuilder {
    id: Option<i64>,
}

impl CreateFolderResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFolderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateFolderResponseBuilder::id)
    pub fn build(self) -> Result<CreateFolderResponse, BuildError> {
        Ok(CreateFolderResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
