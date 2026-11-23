pub use crate::prelude::*;

/// Query parameters for getInboundEmailEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInboundEmailEventsQueryRequest {
    /// Email address of the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// Mandatory if endDate is used. Starting date (YYYY-MM-DD or YYYY-MM-DDTHH:mm:ss.SSSZ) from which you want to fetch the list. Maximum time period that can be selected is 30 days. Must not be in the future.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Mandatory if startDate is used. Ending date (YYYY-MM-DD or YYYY-MM-DDTHH:mm:ss.SSSZ) till which you want to fetch the list. Maximum time period that can be selected is 30 days. Must not be in the future.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of documents returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetInboundEmailEventsRequestSort>,
}

impl GetInboundEmailEventsQueryRequest {
    pub fn builder() -> GetInboundEmailEventsQueryRequestBuilder {
        <GetInboundEmailEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInboundEmailEventsQueryRequestBuilder {
    sender: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetInboundEmailEventsRequestSort>,
}

impl GetInboundEmailEventsQueryRequestBuilder {
    pub fn sender(mut self, value: impl Into<String>) -> Self {
        self.sender = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetInboundEmailEventsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetInboundEmailEventsQueryRequest`].
    pub fn build(self) -> Result<GetInboundEmailEventsQueryRequest, BuildError> {
        Ok(GetInboundEmailEventsQueryRequest {
            sender: self.sender,
            start_date: self.start_date,
            end_date: self.end_date,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
        })
    }
}
