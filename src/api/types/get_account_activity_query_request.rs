pub use crate::prelude::*;

/// Query parameters for getAccountActivity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountActivityQueryRequest {
    /// Mandatory if endDate is used. Enter start date in UTC date (YYYY-MM-DD)
    /// format to filter the activity in your account. Maximum time period that
    /// can be selected is one month. Additionally, you can retrieve activity
    /// logs from the past 12 months from the date of your search.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Mandatory if startDate is used. Enter end date in UTC date (YYYY-MM-DD)
    /// format to filter the activity in your account. Maximum time period that
    /// can be selected is one month.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Enter the user's email address to filter their activity in the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl GetAccountActivityQueryRequest {
    pub fn builder() -> GetAccountActivityQueryRequestBuilder {
        <GetAccountActivityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountActivityQueryRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    email: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl GetAccountActivityQueryRequestBuilder {
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetAccountActivityQueryRequest`].
    pub fn build(self) -> Result<GetAccountActivityQueryRequest, BuildError> {
        Ok(GetAccountActivityQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            email: self.email,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
