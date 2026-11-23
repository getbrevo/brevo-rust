pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct RemoveContactFromListRequestBodyIds {
    /// **Required if 'all' is false and 'emails', 'extIds' are empty.** IDs to remove from a list. You can pass a **maximum of 150 IDs** for removal in one request.
    pub ids: Option<Vec<i64>>,
}

impl RemoveContactFromListRequestBodyIds {
    pub fn builder() -> RemoveContactFromListRequestBodyIdsBuilder {
        <RemoveContactFromListRequestBodyIdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveContactFromListRequestBodyIdsBuilder {
    ids: Option<Vec<i64>>,
}

impl RemoveContactFromListRequestBodyIdsBuilder {
    pub fn ids(mut self, value: Vec<i64>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveContactFromListRequestBodyIds`].
    pub fn build(self) -> Result<RemoveContactFromListRequestBodyIds, BuildError> {
        Ok(RemoveContactFromListRequestBodyIds { ids: self.ids })
    }
}
