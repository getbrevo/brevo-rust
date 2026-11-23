pub use crate::prelude::*;

/// Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBatchEventsRequestEventsItemObject {
    /// Identifiers for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<CreateBatchEventsRequestEventsItemObjectIdentifiers>,
    /// Type of object (e.g., subscription, vehicle, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CreateBatchEventsRequestEventsItemObject {
    pub fn builder() -> CreateBatchEventsRequestEventsItemObjectBuilder {
        <CreateBatchEventsRequestEventsItemObjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchEventsRequestEventsItemObjectBuilder {
    identifiers: Option<CreateBatchEventsRequestEventsItemObjectIdentifiers>,
    r#type: Option<String>,
}

impl CreateBatchEventsRequestEventsItemObjectBuilder {
    pub fn identifiers(
        mut self,
        value: CreateBatchEventsRequestEventsItemObjectIdentifiers,
    ) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchEventsRequestEventsItemObject`].
    pub fn build(self) -> Result<CreateBatchEventsRequestEventsItemObject, BuildError> {
        Ok(CreateBatchEventsRequestEventsItemObject {
            identifiers: self.identifiers,
            r#type: self.r#type,
        })
    }
}
