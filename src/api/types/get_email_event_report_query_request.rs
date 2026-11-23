pub use crate::prelude::*;

/// Query parameters for getEmailEventReport
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailEventReportQueryRequest {
    /// Number limitation for the result returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Beginning point in the list to retrieve from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of days in the past including today (positive integer, maximum 90). _Not compatible with 'startDate' and 'endDate'_. Defaults to 30 if neither dates nor days are provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Filter the report for a specific email addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Filter the report for a specific event type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<GetEmailEventReportRequestEvent>,
    /// Filter the report for tags (serialized and urlencoded array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Filter on a specific message id
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Filter on a specific template id
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetEmailEventReportRequestSort>,
}

impl GetEmailEventReportQueryRequest {
    pub fn builder() -> GetEmailEventReportQueryRequestBuilder {
        <GetEmailEventReportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailEventReportQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    start_date: Option<String>,
    end_date: Option<String>,
    days: Option<i64>,
    email: Option<String>,
    event: Option<GetEmailEventReportRequestEvent>,
    tags: Option<String>,
    message_id: Option<String>,
    template_id: Option<i64>,
    sort: Option<GetEmailEventReportRequestSort>,
}

impl GetEmailEventReportQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
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

    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn event(mut self, value: GetEmailEventReportRequestEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn tags(mut self, value: impl Into<String>) -> Self {
        self.tags = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn sort(mut self, value: GetEmailEventReportRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailEventReportQueryRequest`].
    pub fn build(self) -> Result<GetEmailEventReportQueryRequest, BuildError> {
        Ok(GetEmailEventReportQueryRequest {
            limit: self.limit,
            offset: self.offset,
            start_date: self.start_date,
            end_date: self.end_date,
            days: self.days,
            email: self.email,
            event: self.event,
            tags: self.tags,
            message_id: self.message_id,
            template_id: self.template_id,
            sort: self.sort,
        })
    }
}
