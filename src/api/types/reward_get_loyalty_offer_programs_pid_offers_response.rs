pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyOfferProgramsPidOffersResponse {
    /// Items for the current reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GetLoyaltyOfferProgramsPidOffersResponseItemsItem>>,
    /// Count of the associated parameter in current reward
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl GetLoyaltyOfferProgramsPidOffersResponse {
    pub fn builder() -> GetLoyaltyOfferProgramsPidOffersResponseBuilder {
        <GetLoyaltyOfferProgramsPidOffersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidOffersResponseBuilder {
    items: Option<Vec<GetLoyaltyOfferProgramsPidOffersResponseItemsItem>>,
    total_count: Option<i64>,
}

impl GetLoyaltyOfferProgramsPidOffersResponseBuilder {
    pub fn items(mut self, value: Vec<GetLoyaltyOfferProgramsPidOffersResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidOffersResponse`].
    pub fn build(self) -> Result<GetLoyaltyOfferProgramsPidOffersResponse, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidOffersResponse {
            items: self.items,
            total_count: self.total_count,
        })
    }
}
