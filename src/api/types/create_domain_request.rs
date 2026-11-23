pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainRequest {
    /// Domain name to be added
    #[serde(default)]
    pub name: String,
}

impl CreateDomainRequest {
    pub fn builder() -> CreateDomainRequestBuilder {
        <CreateDomainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainRequestBuilder {
    name: Option<String>,
}

impl CreateDomainRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDomainRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateDomainRequestBuilder::name)
    pub fn build(self) -> Result<CreateDomainRequest, BuildError> {
        Ok(CreateDomainRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
