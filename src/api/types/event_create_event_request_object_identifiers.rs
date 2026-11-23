pub use crate::prelude::*;

/// Identifiers for the object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEventRequestObjectIdentifiers {
    /// External object ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// Internal object ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateEventRequestObjectIdentifiers {
    pub fn builder() -> CreateEventRequestObjectIdentifiersBuilder {
        <CreateEventRequestObjectIdentifiersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEventRequestObjectIdentifiersBuilder {
    ext_id: Option<String>,
    id: Option<String>,
}

impl CreateEventRequestObjectIdentifiersBuilder {
    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEventRequestObjectIdentifiers`].
    pub fn build(self) -> Result<CreateEventRequestObjectIdentifiers, BuildError> {
        Ok(CreateEventRequestObjectIdentifiers {
            ext_id: self.ext_id,
            id: self.id,
        })
    }
}
