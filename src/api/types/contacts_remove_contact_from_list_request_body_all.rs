pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct RemoveContactFromListRequestBodyAll {
    /// **Required if 'emails', 'extIds' and 'ids' are empty.** Remove all existing contacts from a list. A process will be created in this scenario. You can fetch the process details to know about the progress
    pub all: Option<bool>,
}

impl RemoveContactFromListRequestBodyAll {
    pub fn builder() -> RemoveContactFromListRequestBodyAllBuilder {
        <RemoveContactFromListRequestBodyAllBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveContactFromListRequestBodyAllBuilder {
    all: Option<bool>,
}

impl RemoveContactFromListRequestBodyAllBuilder {
    pub fn all(mut self, value: bool) -> Self {
        self.all = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveContactFromListRequestBodyAll`].
    pub fn build(self) -> Result<RemoveContactFromListRequestBodyAll, BuildError> {
        Ok(RemoveContactFromListRequestBodyAll { all: self.all })
    }
}
