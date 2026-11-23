pub use crate::prelude::*;

/// Query parameters for getContactsFromList
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactsFromListQueryRequest {
    /// Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since: Option<String>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetContactsFromListRequestSort>,
}

impl GetContactsFromListQueryRequest {
    pub fn builder() -> GetContactsFromListQueryRequestBuilder {
        <GetContactsFromListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactsFromListQueryRequestBuilder {
    modified_since: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetContactsFromListRequestSort>,
}

impl GetContactsFromListQueryRequestBuilder {
    pub fn modified_since(mut self, value: impl Into<String>) -> Self {
        self.modified_since = Some(value.into());
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

    pub fn sort(mut self, value: GetContactsFromListRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactsFromListQueryRequest`].
    pub fn build(self) -> Result<GetContactsFromListQueryRequest, BuildError> {
        Ok(GetContactsFromListQueryRequest {
            modified_since: self.modified_since,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
        })
    }
}
