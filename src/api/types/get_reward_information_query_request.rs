pub use crate::prelude::*;

/// Query parameters for getRewardInformation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRewardInformationQueryRequest {
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetLoyaltyOfferProgramsPidRewardsRidRequestVersion>,
}

impl GetRewardInformationQueryRequest {
    pub fn builder() -> GetRewardInformationQueryRequestBuilder {
        <GetRewardInformationQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRewardInformationQueryRequestBuilder {
    version: Option<GetLoyaltyOfferProgramsPidRewardsRidRequestVersion>,
}

impl GetRewardInformationQueryRequestBuilder {
    pub fn version(mut self, value: GetLoyaltyOfferProgramsPidRewardsRidRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRewardInformationQueryRequest`].
    pub fn build(self) -> Result<GetRewardInformationQueryRequest, BuildError> {
        Ok(GetRewardInformationQueryRequest {
            version: self.version,
        })
    }
}
