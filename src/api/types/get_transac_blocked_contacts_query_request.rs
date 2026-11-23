pub use crate::prelude::*;

/// Query parameters for getTransacBlockedContacts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransacBlockedContactsQueryRequest {
    /// **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) from which you want to fetch the blocked or unsubscribed contacts
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) till which you want to fetch the blocked or unsubscribed contacts
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of documents returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Comma separated list of emails of the senders from which contacts are blocked or unsubscribed
    #[serde(default)]
    pub senders: Vec<Option<String>>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetTransacBlockedContactsRequestSort>,
}

impl GetTransacBlockedContactsQueryRequest {
    pub fn builder() -> GetTransacBlockedContactsQueryRequestBuilder {
        <GetTransacBlockedContactsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransacBlockedContactsQueryRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    senders: Option<Vec<Option<String>>>,
    sort: Option<GetTransacBlockedContactsRequestSort>,
}

impl GetTransacBlockedContactsQueryRequestBuilder {
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

    pub fn senders(mut self, value: Vec<Option<String>>) -> Self {
        self.senders = Some(value);
        self
    }

    pub fn sort(mut self, value: GetTransacBlockedContactsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTransacBlockedContactsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`senders`](GetTransacBlockedContactsQueryRequestBuilder::senders)
    pub fn build(self) -> Result<GetTransacBlockedContactsQueryRequest, BuildError> {
        Ok(GetTransacBlockedContactsQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            limit: self.limit,
            offset: self.offset,
            senders: self
                .senders
                .ok_or_else(|| BuildError::missing_field("senders"))?,
            sort: self.sort,
        })
    }
}
