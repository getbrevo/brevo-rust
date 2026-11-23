pub use crate::prelude::*;

/// Query parameters for getWhatsappEventReport
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsappEventReportQueryRequest {
    /// Number limitation for the result returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Beginning point in the list to retrieve from
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
    /// Number of days in the past including today (positive integer). _Not compatible with 'startDate' and 'endDate'_
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Filter results for specific contact (WhatsApp Number with country code. Example, 85264318721)
    #[serde(rename = "contactNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_number: Option<String>,
    /// Filter the report for a specific event type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<GetWhatsappEventReportRequestEvent>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetWhatsappEventReportRequestSort>,
}

impl GetWhatsappEventReportQueryRequest {
    pub fn builder() -> GetWhatsappEventReportQueryRequestBuilder {
        <GetWhatsappEventReportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsappEventReportQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    start_date: Option<String>,
    end_date: Option<String>,
    days: Option<i64>,
    contact_number: Option<String>,
    event: Option<GetWhatsappEventReportRequestEvent>,
    sort: Option<GetWhatsappEventReportRequestSort>,
}

impl GetWhatsappEventReportQueryRequestBuilder {
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

    pub fn contact_number(mut self, value: impl Into<String>) -> Self {
        self.contact_number = Some(value.into());
        self
    }

    pub fn event(mut self, value: GetWhatsappEventReportRequestEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn sort(mut self, value: GetWhatsappEventReportRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsappEventReportQueryRequest`].
    pub fn build(self) -> Result<GetWhatsappEventReportQueryRequest, BuildError> {
        Ok(GetWhatsappEventReportQueryRequest {
            limit: self.limit,
            offset: self.offset,
            start_date: self.start_date,
            end_date: self.end_date,
            days: self.days,
            contact_number: self.contact_number,
            event: self.event,
            sort: self.sort,
        })
    }
}
