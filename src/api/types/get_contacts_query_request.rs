pub use crate::prelude::*;

/// Query parameters for getContacts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactsQueryRequest {
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since: Option<String>,
    /// Filter (urlencoded) the contacts created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "createdSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_since: Option<String>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetContactsRequestSort>,
    /// Filter by a list of contact IDs. You can pass a **maximum of 20 IDs**. All elements must be integers.
    #[serde(default)]
    pub ids: Vec<Option<i64>>,
    /// Id of the segment. **Either listIds or segmentId can be passed.** Must be a positive integer (minimum value of 1).
    #[serde(rename = "segmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<i64>,
    /// Ids of the list. **Either listIds or segmentId can be passed.**
    #[serde(rename = "listIds")]
    #[serde(default)]
    pub list_ids: Vec<Option<i64>>,
    /// Filter the contacts on the basis of attributes. **Allowed operator: equals. For multiple-choice options, the filter will apply an AND condition between the options. For category attributes, the filter will work with both id and value. (e.g. filter=equals(FIRSTNAME,"Antoine"), filter=equals(B1, true), filter=equals(DOB, "1989-11-23"), filter=equals(GENDER, "1"), filter=equals(GENDER, "MALE"), filter=equals(COUNTRY,"USA, INDIA")**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

impl GetContactsQueryRequest {
    pub fn builder() -> GetContactsQueryRequestBuilder {
        <GetContactsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    modified_since: Option<String>,
    created_since: Option<String>,
    sort: Option<GetContactsRequestSort>,
    ids: Option<Vec<Option<i64>>>,
    segment_id: Option<i64>,
    list_ids: Option<Vec<Option<i64>>>,
    filter: Option<String>,
}

impl GetContactsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn modified_since(mut self, value: impl Into<String>) -> Self {
        self.modified_since = Some(value.into());
        self
    }

    pub fn created_since(mut self, value: impl Into<String>) -> Self {
        self.created_since = Some(value.into());
        self
    }

    pub fn sort(mut self, value: GetContactsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn ids(mut self, value: Vec<Option<i64>>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn segment_id(mut self, value: i64) -> Self {
        self.segment_id = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<Option<i64>>) -> Self {
        self.list_ids = Some(value);
        self
    }

    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ids`](GetContactsQueryRequestBuilder::ids)
    /// - [`list_ids`](GetContactsQueryRequestBuilder::list_ids)
    pub fn build(self) -> Result<GetContactsQueryRequest, BuildError> {
        Ok(GetContactsQueryRequest {
            limit: self.limit,
            offset: self.offset,
            modified_since: self.modified_since,
            created_since: self.created_since,
            sort: self.sort,
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
            segment_id: self.segment_id,
            list_ids: self
                .list_ids
                .ok_or_else(|| BuildError::missing_field("list_ids"))?,
            filter: self.filter,
        })
    }
}
