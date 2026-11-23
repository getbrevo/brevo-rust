pub use crate::prelude::*;

/// Query parameters for getAllCompanies
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllCompaniesQueryRequest {
    /// Filter by attributes. If you have a filter for the owner on your side please send it as filters[attributes.owner] and utilize the account email for the filtering.
    #[serde(rename = "filters[attributes.name]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_attributes_name: Option<String>,
    /// Filter by linked contacts ids
    #[serde(rename = "linkedContactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_contacts_ids: Option<i64>,
    /// Filter by linked Deals ids
    #[serde(rename = "linkedDealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_deals_ids: Option<String>,
    /// Filter (urlencoded) the companies modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since: Option<String>,
    /// Filter (urlencoded) the companies created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    #[serde(rename = "createdSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_since: Option<String>,
    /// Index of the first document of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetCompaniesRequestSort>,
    /// The field used to sort field names.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl GetAllCompaniesQueryRequest {
    pub fn builder() -> GetAllCompaniesQueryRequestBuilder {
        <GetAllCompaniesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllCompaniesQueryRequestBuilder {
    filters_attributes_name: Option<String>,
    linked_contacts_ids: Option<i64>,
    linked_deals_ids: Option<String>,
    modified_since: Option<String>,
    created_since: Option<String>,
    page: Option<i64>,
    limit: Option<i64>,
    sort: Option<GetCompaniesRequestSort>,
    sort_by: Option<String>,
}

impl GetAllCompaniesQueryRequestBuilder {
    pub fn filters_attributes_name(mut self, value: impl Into<String>) -> Self {
        self.filters_attributes_name = Some(value.into());
        self
    }

    pub fn linked_contacts_ids(mut self, value: i64) -> Self {
        self.linked_contacts_ids = Some(value);
        self
    }

    pub fn linked_deals_ids(mut self, value: impl Into<String>) -> Self {
        self.linked_deals_ids = Some(value.into());
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

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: GetCompaniesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAllCompaniesQueryRequest`].
    pub fn build(self) -> Result<GetAllCompaniesQueryRequest, BuildError> {
        Ok(GetAllCompaniesQueryRequest {
            filters_attributes_name: self.filters_attributes_name,
            linked_contacts_ids: self.linked_contacts_ids,
            linked_deals_ids: self.linked_deals_ids,
            modified_since: self.modified_since,
            created_since: self.created_since,
            page: self.page,
            limit: self.limit,
            sort: self.sort,
            sort_by: self.sort_by,
        })
    }
}
