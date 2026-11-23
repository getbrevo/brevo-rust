pub use crate::prelude::*;

/// Query parameters for getBalanceLimit
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBalanceLimitQueryRequest {
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetBalanceLimitRequestVersion>,
}

impl GetBalanceLimitQueryRequest {
    pub fn builder() -> GetBalanceLimitQueryRequestBuilder {
        <GetBalanceLimitQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBalanceLimitQueryRequestBuilder {
    version: Option<GetBalanceLimitRequestVersion>,
}

impl GetBalanceLimitQueryRequestBuilder {
    pub fn version(mut self, value: GetBalanceLimitRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBalanceLimitQueryRequest`].
    pub fn build(self) -> Result<GetBalanceLimitQueryRequest, BuildError> {
        Ok(GetBalanceLimitQueryRequest {
            version: self.version,
        })
    }
}
