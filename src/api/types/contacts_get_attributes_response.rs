pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAttributesResponse {
    /// Listing of available contact attributes in your account
    #[serde(default)]
    pub attributes: Vec<GetAttributesResponseAttributesItem>,
}

impl GetAttributesResponse {
    pub fn builder() -> GetAttributesResponseBuilder {
        <GetAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAttributesResponseBuilder {
    attributes: Option<Vec<GetAttributesResponseAttributesItem>>,
}

impl GetAttributesResponseBuilder {
    pub fn attributes(mut self, value: Vec<GetAttributesResponseAttributesItem>) -> Self {
        self.attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](GetAttributesResponseBuilder::attributes)
    pub fn build(self) -> Result<GetAttributesResponse, BuildError> {
        Ok(GetAttributesResponse {
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
        })
    }
}
