pub use crate::prelude::*;

/// Query parameters for getTransacEmailsList
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacEmailsListQueryRequest {
    /// **Mandatory if templateId and messageId are not passed in query filters.** Email address to which transactional email has been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// **Mandatory if email and messageId are not passed in query filters.** Id of the template that was used to compose transactional email.
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// **Mandatory if templateId and email are not passed in query filters.** Message ID of the transactional email sent.
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) from which you want to fetch the list. **Maximum time period that can be selected is one month**.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) till which you want to fetch the list. **Maximum time period that can be selected is one month.**
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetTransacEmailsListRequestSort>,
    /// Number of documents returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl GetTransacEmailsListQueryRequest {
    pub fn builder() -> GetTransacEmailsListQueryRequestBuilder {
        <GetTransacEmailsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacEmailsListQueryRequestBuilder {
    email: Option<String>,
    template_id: Option<i64>,
    message_id: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    sort: Option<GetTransacEmailsListRequestSort>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl GetTransacEmailsListQueryRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
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

    pub fn sort(mut self, value: GetTransacEmailsListRequestSort) -> Self {
        self.sort = Some(value);
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

    /// Consumes the builder and constructs a [`GetTransacEmailsListQueryRequest`].
    pub fn build(self) -> Result<GetTransacEmailsListQueryRequest, BuildError> {
        Ok(GetTransacEmailsListQueryRequest {
            email: self.email,
            template_id: self.template_id,
            message_id: self.message_id,
            start_date: self.start_date,
            end_date: self.end_date,
            sort: self.sort,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
