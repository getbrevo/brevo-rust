pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct RemoveContactFromListRequestBodyExtIds {
    /// **Required if 'all' is false, 'ids' and 'emails' are empty.** EXT_ID attributes to remove from a list. You can pass a **maximum of 150 EXT_ID attributes** for removal in one request.
    pub ext_ids: Option<Vec<String>>,
}

impl RemoveContactFromListRequestBodyExtIds {
    pub fn builder() -> RemoveContactFromListRequestBodyExtIdsBuilder {
        <RemoveContactFromListRequestBodyExtIdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveContactFromListRequestBodyExtIdsBuilder {
    ext_ids: Option<Vec<String>>,
}

impl RemoveContactFromListRequestBodyExtIdsBuilder {
    pub fn ext_ids(mut self, value: Vec<String>) -> Self {
        self.ext_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveContactFromListRequestBodyExtIds`].
    pub fn build(self) -> Result<RemoveContactFromListRequestBodyExtIds, BuildError> {
        Ok(RemoveContactFromListRequestBodyExtIds {
            ext_ids: self.ext_ids,
        })
    }
}
