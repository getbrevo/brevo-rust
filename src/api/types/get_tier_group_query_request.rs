pub use crate::prelude::*;

/// Query parameters for getTierGroup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTierGroupQueryRequest {
    /// Select active to retrieve active version of tier group. Select draft to retrieve latest changes in tier group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetTierGroupRequestVersion>,
}

impl GetTierGroupQueryRequest {
    pub fn builder() -> GetTierGroupQueryRequestBuilder {
        <GetTierGroupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTierGroupQueryRequestBuilder {
    version: Option<GetTierGroupRequestVersion>,
}

impl GetTierGroupQueryRequestBuilder {
    pub fn version(mut self, value: GetTierGroupRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTierGroupQueryRequest`].
    pub fn build(self) -> Result<GetTierGroupQueryRequest, BuildError> {
        Ok(GetTierGroupQueryRequest {
            version: self.version,
        })
    }
}
