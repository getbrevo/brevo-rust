pub use crate::prelude::*;

/// Set of attributes of the contact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactDetailsAttributes {}

impl GetContactDetailsAttributes {
    pub fn builder() -> GetContactDetailsAttributesBuilder {
        <GetContactDetailsAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactDetailsAttributesBuilder {}

impl GetContactDetailsAttributesBuilder {
    /// Consumes the builder and constructs a [`GetContactDetailsAttributes`].
    pub fn build(self) -> Result<GetContactDetailsAttributes, BuildError> {
        Ok(GetContactDetailsAttributes {})
    }
}
