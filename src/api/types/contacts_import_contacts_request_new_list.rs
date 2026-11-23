pub use crate::prelude::*;

/// To create a new list and import the contacts into it, pass the listName and an optional folderId.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportContactsRequestNewList {
    /// Id of the folder where this new list shall be created. **Mandatory if listName is not empty**
    #[serde(rename = "folderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    /// List with listName will be created first and users will be imported in it. **Mandatory if listIds is empty**.
    #[serde(rename = "listName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
}

impl ImportContactsRequestNewList {
    pub fn builder() -> ImportContactsRequestNewListBuilder {
        <ImportContactsRequestNewListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportContactsRequestNewListBuilder {
    folder_id: Option<i64>,
    list_name: Option<String>,
}

impl ImportContactsRequestNewListBuilder {
    pub fn folder_id(mut self, value: i64) -> Self {
        self.folder_id = Some(value);
        self
    }

    pub fn list_name(mut self, value: impl Into<String>) -> Self {
        self.list_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportContactsRequestNewList`].
    pub fn build(self) -> Result<ImportContactsRequestNewList, BuildError> {
        Ok(ImportContactsRequestNewList {
            folder_id: self.folder_id,
            list_name: self.list_name,
        })
    }
}
