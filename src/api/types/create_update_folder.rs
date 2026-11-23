pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateFolder {
    /// Name of the folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateUpdateFolder {
    pub fn builder() -> CreateUpdateFolderBuilder {
        <CreateUpdateFolderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateFolderBuilder {
    name: Option<String>,
}

impl CreateUpdateFolderBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateFolder`].
    pub fn build(self) -> Result<CreateUpdateFolder, BuildError> {
        Ok(CreateUpdateFolder { name: self.name })
    }
}
