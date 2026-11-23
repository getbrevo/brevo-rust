pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateContactResponse {
    /// ID of the contact when a new contact is created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl CreateContactResponse {
    pub fn builder() -> CreateContactResponseBuilder {
        <CreateContactResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateContactResponseBuilder {
    id: Option<i64>,
}

impl CreateContactResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateContactResponse`].
    pub fn build(self) -> Result<CreateContactResponse, BuildError> {
        Ok(CreateContactResponse { id: self.id })
    }
}
