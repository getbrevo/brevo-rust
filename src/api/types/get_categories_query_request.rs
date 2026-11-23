pub use crate::prelude::*;

/// Query parameters for getCategories
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCategoriesQueryRequest {
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetCategoriesRequestSort>,
    /// Filter by category ids
    #[serde(default)]
    pub ids: Vec<Option<String>>,
    /// Filter by category name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter (urlencoded) the categories modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since: Option<String>,
    /// Filter (urlencoded) the categories created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "createdSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_since: Option<String>,
    /// Filter categories by their deletion status. If `false` is passed, only categories that are not deleted will be returned.
    #[serde(rename = "isDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<String>,
}

impl GetCategoriesQueryRequest {
    pub fn builder() -> GetCategoriesQueryRequestBuilder {
        <GetCategoriesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCategoriesQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetCategoriesRequestSort>,
    ids: Option<Vec<Option<String>>>,
    name: Option<String>,
    modified_since: Option<String>,
    created_since: Option<String>,
    is_deleted: Option<String>,
}

impl GetCategoriesQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetCategoriesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn is_deleted(mut self, value: impl Into<String>) -> Self {
        self.is_deleted = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCategoriesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ids`](GetCategoriesQueryRequestBuilder::ids)
    pub fn build(self) -> Result<GetCategoriesQueryRequest, BuildError> {
        Ok(GetCategoriesQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
            name: self.name,
            modified_since: self.modified_since,
            created_since: self.created_since,
            is_deleted: self.is_deleted,
        })
    }
}
