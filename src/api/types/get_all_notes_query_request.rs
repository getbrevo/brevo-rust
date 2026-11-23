pub use crate::prelude::*;

/// Query parameters for getAllNotes
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllNotesQueryRequest {
    /// Filter by note entity type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<GetCrmNotesRequestEntity>,
    /// Filter by note entity IDs
    #[serde(rename = "entityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<String>,
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
    pub sort: Option<GetCrmNotesRequestSort>,
}

impl GetAllNotesQueryRequest {
    pub fn builder() -> GetAllNotesQueryRequestBuilder {
        <GetAllNotesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllNotesQueryRequestBuilder {
    entity: Option<GetCrmNotesRequestEntity>,
    entity_ids: Option<String>,
    date_from: Option<i64>,
    date_to: Option<i64>,
    offset: Option<i64>,
    limit: Option<i64>,
    sort: Option<GetCrmNotesRequestSort>,
}

impl GetAllNotesQueryRequestBuilder {
    pub fn entity(mut self, value: GetCrmNotesRequestEntity) -> Self {
        self.entity = Some(value);
        self
    }

    pub fn entity_ids(mut self, value: impl Into<String>) -> Self {
        self.entity_ids = Some(value.into());
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

    pub fn sort(mut self, value: GetCrmNotesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAllNotesQueryRequest`].
    pub fn build(self) -> Result<GetAllNotesQueryRequest, BuildError> {
        Ok(GetAllNotesQueryRequest {
            entity: self.entity,
            entity_ids: self.entity_ids,
            date_from: self.date_from,
            date_to: self.date_to,
            offset: self.offset,
            limit: self.limit,
            sort: self.sort,
        })
    }
}
