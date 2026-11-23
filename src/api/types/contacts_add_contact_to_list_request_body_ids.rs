pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AddContactToListRequestBodyIds {
    /// IDs to add to a list. You can pass a **maximum of 150 IDs** for addition in one request. **_If you need to add the emails in bulk, please prefer /contacts/import api._**
    pub ids: Option<Vec<i64>>,
}

impl AddContactToListRequestBodyIds {
    pub fn builder() -> AddContactToListRequestBodyIdsBuilder {
        <AddContactToListRequestBodyIdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddContactToListRequestBodyIdsBuilder {
    ids: Option<Vec<i64>>,
}

impl AddContactToListRequestBodyIdsBuilder {
    pub fn ids(mut self, value: Vec<i64>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddContactToListRequestBodyIds`].
    pub fn build(self) -> Result<AddContactToListRequestBodyIds, BuildError> {
        Ok(AddContactToListRequestBodyIds { ids: self.ids })
    }
}
