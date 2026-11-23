pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSegmentsResponse {
    /// Number of Segments in your account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Listing of all the segments available in your account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<GetSegmentsResponseSegmentsItem>>,
}

impl GetSegmentsResponse {
    pub fn builder() -> GetSegmentsResponseBuilder {
        <GetSegmentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSegmentsResponseBuilder {
    count: Option<i64>,
    segments: Option<Vec<GetSegmentsResponseSegmentsItem>>,
}

impl GetSegmentsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn segments(mut self, value: Vec<GetSegmentsResponseSegmentsItem>) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSegmentsResponse`].
    pub fn build(self) -> Result<GetSegmentsResponse, BuildError> {
        Ok(GetSegmentsResponse {
            count: self.count,
            segments: self.segments,
        })
    }
}
