pub use crate::prelude::*;

/// Query parameters for getContactStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactStatsQueryRequest {
    /// **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be lower than equal to endDate. Must not be greater than the current date.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be greater than equal to startDate. Must not be greater than the current date. Maximum difference between startDate and endDate should not be greater than 90 days.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl GetContactStatsQueryRequest {
    pub fn builder() -> GetContactStatsQueryRequestBuilder {
        <GetContactStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactStatsQueryRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
}

impl GetContactStatsQueryRequestBuilder {
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactStatsQueryRequest`].
    pub fn build(self) -> Result<GetContactStatsQueryRequest, BuildError> {
        Ok(GetContactStatsQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
        })
    }
}
