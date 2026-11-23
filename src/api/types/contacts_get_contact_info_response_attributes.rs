pub use crate::prelude::*;

/// Set of attributes of the contact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseAttributes {}

impl GetContactInfoResponseAttributes {
    pub fn builder() -> GetContactInfoResponseAttributesBuilder {
        <GetContactInfoResponseAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseAttributesBuilder {}

impl GetContactInfoResponseAttributesBuilder {
    /// Consumes the builder and constructs a [`GetContactInfoResponseAttributes`].
    pub fn build(self) -> Result<GetContactInfoResponseAttributes, BuildError> {
        Ok(GetContactInfoResponseAttributes {})
    }
}
