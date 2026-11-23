pub use crate::prelude::*;

/// Query parameters for getAllTasks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllTasksQueryRequest {
    /// Filter by task type (ID)
    #[serde(rename = "filter[type]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    /// Filter by task status
    #[serde(rename = "filter[status]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_status: Option<GetCrmTasksRequestFilterStatus>,
    /// Filter by date
    #[serde(rename = "filter[date]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_date: Option<GetCrmTasksRequestFilterDate>,
    /// Filter by the "assignTo" ID. You can utilize account emails for the "assignTo" attribute.
    #[serde(rename = "filter[assignTo]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_assign_to: Option<String>,
    /// Filter by contact ids
    #[serde(rename = "filter[contacts]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_contacts: Option<String>,
    /// Filter by deals ids
    #[serde(rename = "filter[deals]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_deals: Option<String>,
    /// Filter by companies ids
    #[serde(rename = "filter[companies]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_companies: Option<String>,
    /// dateFrom to date range filter type (timestamp in milliseconds)
    #[serde(rename = "dateFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<i64>,
    /// dateTo to date range filter type (timestamp in milliseconds)
    #[serde(rename = "dateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<i64>,
    /// Index of the first document of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetCrmTasksRequestSort>,
    /// The field used to sort field names.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl GetAllTasksQueryRequest {
    pub fn builder() -> GetAllTasksQueryRequestBuilder {
        <GetAllTasksQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllTasksQueryRequestBuilder {
    filter_type: Option<String>,
    filter_status: Option<GetCrmTasksRequestFilterStatus>,
    filter_date: Option<GetCrmTasksRequestFilterDate>,
    filter_assign_to: Option<String>,
    filter_contacts: Option<String>,
    filter_deals: Option<String>,
    filter_companies: Option<String>,
    date_from: Option<i64>,
    date_to: Option<i64>,
    offset: Option<i64>,
    limit: Option<i64>,
    sort: Option<GetCrmTasksRequestSort>,
    sort_by: Option<String>,
}

impl GetAllTasksQueryRequestBuilder {
    pub fn filter_type(mut self, value: impl Into<String>) -> Self {
        self.filter_type = Some(value.into());
        self
    }

    pub fn filter_status(mut self, value: GetCrmTasksRequestFilterStatus) -> Self {
        self.filter_status = Some(value);
        self
    }

    pub fn filter_date(mut self, value: GetCrmTasksRequestFilterDate) -> Self {
        self.filter_date = Some(value);
        self
    }

    pub fn filter_assign_to(mut self, value: impl Into<String>) -> Self {
        self.filter_assign_to = Some(value.into());
        self
    }

    pub fn filter_contacts(mut self, value: impl Into<String>) -> Self {
        self.filter_contacts = Some(value.into());
        self
    }

    pub fn filter_deals(mut self, value: impl Into<String>) -> Self {
        self.filter_deals = Some(value.into());
        self
    }

    pub fn filter_companies(mut self, value: impl Into<String>) -> Self {
        self.filter_companies = Some(value.into());
        self
    }

    pub fn date_from(mut self, value: i64) -> Self {
        self.date_from = Some(value);
        self
    }

    pub fn date_to(mut self, value: i64) -> Self {
        self.date_to = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: GetCrmTasksRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAllTasksQueryRequest`].
    pub fn build(self) -> Result<GetAllTasksQueryRequest, BuildError> {
        Ok(GetAllTasksQueryRequest {
            filter_type: self.filter_type,
            filter_status: self.filter_status,
            filter_date: self.filter_date,
            filter_assign_to: self.filter_assign_to,
            filter_contacts: self.filter_contacts,
            filter_deals: self.filter_deals,
            filter_companies: self.filter_companies,
            date_from: self.date_from,
            date_to: self.date_to,
            offset: self.offset,
            limit: self.limit,
            sort: self.sort,
            sort_by: self.sort_by,
        })
    }
}
