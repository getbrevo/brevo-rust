pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllExternalFeedsResponse {
    /// Total number of feeds
    #[serde(default)]
    pub count: i64,
    /// List of external feeds
    #[serde(default)]
    pub feeds: Vec<GetAllExternalFeedsResponseFeedsItem>,
}

impl GetAllExternalFeedsResponse {
    pub fn builder() -> GetAllExternalFeedsResponseBuilder {
        <GetAllExternalFeedsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllExternalFeedsResponseBuilder {
    count: Option<i64>,
    feeds: Option<Vec<GetAllExternalFeedsResponseFeedsItem>>,
}

impl GetAllExternalFeedsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn feeds(mut self, value: Vec<GetAllExternalFeedsResponseFeedsItem>) -> Self {
        self.feeds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAllExternalFeedsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](GetAllExternalFeedsResponseBuilder::count)
    /// - [`feeds`](GetAllExternalFeedsResponseBuilder::feeds)
    pub fn build(self) -> Result<GetAllExternalFeedsResponse, BuildError> {
        Ok(GetAllExternalFeedsResponse {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            feeds: self
                .feeds
                .ok_or_else(|| BuildError::missing_field("feeds"))?,
        })
    }
}
