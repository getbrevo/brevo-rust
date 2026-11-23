pub use crate::prelude::*;

/// Query parameters for getSmsEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsEventsQueryRequest {
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Index of the first document of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Number of days in the past including today (positive integer). **Not compatible with 'startDate' and 'endDate'**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Filter the report for a specific phone number
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Filter the report for specific events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<GetSmsEventsRequestEvent>,
    /// Filter the report for specific tags passed as a serialized urlencoded array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetSmsEventsRequestSort>,
}

impl GetSmsEventsQueryRequest {
    pub fn builder() -> GetSmsEventsQueryRequestBuilder {
        <GetSmsEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsEventsQueryRequestBuilder {
    limit: Option<i64>,
    start_date: Option<String>,
    end_date: Option<String>,
    offset: Option<i64>,
    days: Option<i64>,
    phone_number: Option<String>,
    event: Option<GetSmsEventsRequestEvent>,
    tags: Option<String>,
    sort: Option<GetSmsEventsRequestSort>,
}

impl GetSmsEventsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
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

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn event(mut self, value: GetSmsEventsRequestEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn tags(mut self, value: impl Into<String>) -> Self {
        self.tags = Some(value.into());
        self
    }

    pub fn sort(mut self, value: GetSmsEventsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsEventsQueryRequest`].
    pub fn build(self) -> Result<GetSmsEventsQueryRequest, BuildError> {
        Ok(GetSmsEventsQueryRequest {
            limit: self.limit,
            start_date: self.start_date,
            end_date: self.end_date,
            offset: self.offset,
            days: self.days,
            phone_number: self.phone_number,
            event: self.event,
            tags: self.tags,
            sort: self.sort,
        })
    }
}
