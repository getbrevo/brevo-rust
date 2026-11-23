pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBlockedDomainsResponse {
    /// List of all blocked domains
    #[serde(default)]
    pub domains: Vec<String>,
}

impl GetBlockedDomainsResponse {
    pub fn builder() -> GetBlockedDomainsResponseBuilder {
        <GetBlockedDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBlockedDomainsResponseBuilder {
    domains: Option<Vec<String>>,
}

impl GetBlockedDomainsResponseBuilder {
    pub fn domains(mut self, value: Vec<String>) -> Self {
        self.domains = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBlockedDomainsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domains`](GetBlockedDomainsResponseBuilder::domains)
    pub fn build(self) -> Result<GetBlockedDomainsResponse, BuildError> {
        Ok(GetBlockedDomainsResponse {
            domains: self
                .domains
                .ok_or_else(|| BuildError::missing_field("domains"))?,
        })
    }
}
