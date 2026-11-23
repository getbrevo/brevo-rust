pub use crate::prelude::*;

/// Identifiers for the object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBatchEventsRequestEventsItemObjectIdentifiers {
    /// External object ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// Internal object ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateBatchEventsRequestEventsItemObjectIdentifiers {
    pub fn builder() -> CreateBatchEventsRequestEventsItemObjectIdentifiersBuilder {
        <CreateBatchEventsRequestEventsItemObjectIdentifiersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchEventsRequestEventsItemObjectIdentifiersBuilder {
    ext_id: Option<String>,
    id: Option<String>,
}

impl CreateBatchEventsRequestEventsItemObjectIdentifiersBuilder {
    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchEventsRequestEventsItemObjectIdentifiers`].
    pub fn build(self) -> Result<CreateBatchEventsRequestEventsItemObjectIdentifiers, BuildError> {
        Ok(CreateBatchEventsRequestEventsItemObjectIdentifiers {
            ext_id: self.ext_id,
            id: self.id,
        })
    }
}
