pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BlockNewDomainRequest {
    /// name of the domain to be blocked
    #[serde(default)]
    pub domain: String,
}

impl BlockNewDomainRequest {
    pub fn builder() -> BlockNewDomainRequestBuilder {
        <BlockNewDomainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BlockNewDomainRequestBuilder {
    domain: Option<String>,
}

impl BlockNewDomainRequestBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BlockNewDomainRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](BlockNewDomainRequestBuilder::domain)
    pub fn build(self) -> Result<BlockNewDomainRequest, BuildError> {
        Ok(BlockNewDomainRequest {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
        })
    }
}
