pub use crate::prelude::*;

/// Query parameters for getListOfTierGroups
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetListOfTierGroupsQueryRequest {
    /// Select 'active' to retrieve list of all tier groups which are live for clients. Select draft to retrieve list of all non deleted tier groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetListOfTierGroupsRequestVersion>,
}

impl GetListOfTierGroupsQueryRequest {
    pub fn builder() -> GetListOfTierGroupsQueryRequestBuilder {
        <GetListOfTierGroupsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetListOfTierGroupsQueryRequestBuilder {
    version: Option<GetListOfTierGroupsRequestVersion>,
}

impl GetListOfTierGroupsQueryRequestBuilder {
    pub fn version(mut self, value: GetListOfTierGroupsRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetListOfTierGroupsQueryRequest`].
    pub fn build(self) -> Result<GetListOfTierGroupsQueryRequest, BuildError> {
        Ok(GetListOfTierGroupsQueryRequest {
            version: self.version,
        })
    }
}
