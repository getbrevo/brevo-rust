pub use crate::prelude::*;

/// Query parameters for getBalanceDefinition
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBalanceDefinitionQueryRequest {
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetBalanceDefinitionRequestVersion>,
}

impl GetBalanceDefinitionQueryRequest {
    pub fn builder() -> GetBalanceDefinitionQueryRequestBuilder {
        <GetBalanceDefinitionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBalanceDefinitionQueryRequestBuilder {
    version: Option<GetBalanceDefinitionRequestVersion>,
}

impl GetBalanceDefinitionQueryRequestBuilder {
    pub fn version(mut self, value: GetBalanceDefinitionRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBalanceDefinitionQueryRequest`].
    pub fn build(self) -> Result<GetBalanceDefinitionQueryRequest, BuildError> {
        Ok(GetBalanceDefinitionQueryRequest {
            version: self.version,
        })
    }
}
