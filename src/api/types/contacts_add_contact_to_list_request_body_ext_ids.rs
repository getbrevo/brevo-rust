pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AddContactToListRequestBodyExtIds {
    /// EXT_ID attributes to add to a list. You can pass a **maximum of 150 EXT_ID attributes** for addition in one request. **_If you need to add the emails in bulk, please prefer /contacts/import api._**
    pub ext_ids: Option<Vec<String>>,
}

impl AddContactToListRequestBodyExtIds {
    pub fn builder() -> AddContactToListRequestBodyExtIdsBuilder {
        <AddContactToListRequestBodyExtIdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddContactToListRequestBodyExtIdsBuilder {
    ext_ids: Option<Vec<String>>,
}

impl AddContactToListRequestBodyExtIdsBuilder {
    pub fn ext_ids(mut self, value: Vec<String>) -> Self {
        self.ext_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddContactToListRequestBodyExtIds`].
    pub fn build(self) -> Result<AddContactToListRequestBodyExtIds, BuildError> {
        Ok(AddContactToListRequestBodyExtIds {
            ext_ids: self.ext_ids,
        })
    }
}
