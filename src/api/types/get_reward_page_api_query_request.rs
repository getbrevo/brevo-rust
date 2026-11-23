pub use crate::prelude::*;

/// Query parameters for getRewardPageApi
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRewardPageApiQueryRequest {
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Pagination offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// State of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<GetLoyaltyOfferProgramsPidOffersRequestVersion>,
}

impl GetRewardPageApiQueryRequest {
    pub fn builder() -> GetRewardPageApiQueryRequestBuilder {
        <GetRewardPageApiQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRewardPageApiQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    state: Option<String>,
    version: Option<GetLoyaltyOfferProgramsPidOffersRequestVersion>,
}

impl GetRewardPageApiQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn version(mut self, value: GetLoyaltyOfferProgramsPidOffersRequestVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRewardPageApiQueryRequest`].
    pub fn build(self) -> Result<GetRewardPageApiQueryRequest, BuildError> {
        Ok(GetRewardPageApiQueryRequest {
            limit: self.limit,
            offset: self.offset,
            state: self.state,
            version: self.version,
        })
    }
}
