pub use crate::prelude::*;

/// Query parameters for getLoyaltyProgramTier
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyProgramTierQueryRequest {
    /// Select 'active' to retrieve list of all tiers which are live for clients. Select draft to retrieve list of all non deleted tiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetLoyaltyProgramTierRequestVersion>,
}

impl GetLoyaltyProgramTierQueryRequest {
    pub fn builder() -> GetLoyaltyProgramTierQueryRequestBuilder {
        <GetLoyaltyProgramTierQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyProgramTierQueryRequestBuilder {
    version: Option<GetLoyaltyProgramTierRequestVersion>,
}

impl GetLoyaltyProgramTierQueryRequestBuilder {
    pub fn version(mut self, value: GetLoyaltyProgramTierRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyProgramTierQueryRequest`].
    pub fn build(self) -> Result<GetLoyaltyProgramTierQueryRequest, BuildError> {
        Ok(GetLoyaltyProgramTierQueryRequest {
            version: self.version,
        })
    }
}
