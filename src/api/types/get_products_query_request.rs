pub use crate::prelude::*;

/// Query parameters for getProducts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetProductsQueryRequest {
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetProductsRequestSort>,
    /// Filter by product ids
    #[serde(default)]
    pub ids: Vec<Option<String>>,
    /// Search products simultaneously across SKU, name, and ID fields. Results are returned in the following priority order: **exact SKU match** > **SKU prefix match** > **name match** > **ID match**. For example, `?search=123` on products with `{sku: "123"}` and `{sku: "123456"}` returns the exact SKU match first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by product name, minimum 3 characters should be present for search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Price filter for products less than and equals to particular amount
    #[serde(rename = "price[lte]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_lte: Option<f64>,
    /// Price filter for products greater than and equals to particular amount
    #[serde(rename = "price[gte]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_gte: Option<f64>,
    /// Price filter for products less than particular amount
    #[serde(rename = "price[lt]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_lt: Option<f64>,
    /// Price filter for products greater than particular amount
    #[serde(rename = "price[gt]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_gt: Option<f64>,
    /// Price filter for products equals to particular amount
    #[serde(rename = "price[eq]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_eq: Option<f64>,
    /// Price filter for products not equals to particular amount
    #[serde(rename = "price[ne]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_ne: Option<f64>,
    /// Alternative price filter for products less than and equals to particular amount
    #[serde(rename = "alternativePrice[lte]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alternative_price_lte: Option<f64>,
    /// Alternative price filter for products greater than and equals to particular amount
    #[serde(rename = "alternativePrice[gte]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alternative_price_gte: Option<f64>,
    /// Alternative price filter for products less than particular amount
    #[serde(rename = "alternativePrice[lt]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alternative_price_lt: Option<f64>,
    /// Alternative price filter for products greater than particular amount
    #[serde(rename = "alternativePrice[gt]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alternative_price_gt: Option<f64>,
    /// Alternative price filter for products equals to particular amount
    #[serde(rename = "alternativePrice[eq]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alternative_price_eq: Option<f64>,
    /// Alternative price filter for products not equals to particular amount
    #[serde(rename = "alternativePrice[ne]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alternative_price_ne: Option<f64>,
    /// Filter by categories ids
    #[serde(default)]
    pub categories: Vec<Option<String>>,
    /// Filter (urlencoded) the products modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since: Option<String>,
    /// Filter (urlencoded) the products created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    #[serde(rename = "createdSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_since: Option<String>,
    /// Sort the results by a specific field. Default sort field is `created_at` when not passed.
    #[serde(rename = "sortByField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_field: Option<GetProductsRequestSortByField>,
    /// Filter products by their deletion status. If `false` is passed, only products that are not deleted will be returned.
    #[serde(rename = "isDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<String>,
}

impl GetProductsQueryRequest {
    pub fn builder() -> GetProductsQueryRequestBuilder {
        <GetProductsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProductsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetProductsRequestSort>,
    ids: Option<Vec<Option<String>>>,
    search: Option<String>,
    name: Option<String>,
    price_lte: Option<f64>,
    price_gte: Option<f64>,
    price_lt: Option<f64>,
    price_gt: Option<f64>,
    price_eq: Option<f64>,
    price_ne: Option<f64>,
    alternative_price_lte: Option<f64>,
    alternative_price_gte: Option<f64>,
    alternative_price_lt: Option<f64>,
    alternative_price_gt: Option<f64>,
    alternative_price_eq: Option<f64>,
    alternative_price_ne: Option<f64>,
    categories: Option<Vec<Option<String>>>,
    modified_since: Option<String>,
    created_since: Option<String>,
    sort_by_field: Option<GetProductsRequestSortByField>,
    is_deleted: Option<String>,
}

impl GetProductsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetProductsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn price_lte(mut self, value: f64) -> Self {
        self.price_lte = Some(value);
        self
    }

    pub fn price_gte(mut self, value: f64) -> Self {
        self.price_gte = Some(value);
        self
    }

    pub fn price_lt(mut self, value: f64) -> Self {
        self.price_lt = Some(value);
        self
    }

    pub fn price_gt(mut self, value: f64) -> Self {
        self.price_gt = Some(value);
        self
    }

    pub fn price_eq(mut self, value: f64) -> Self {
        self.price_eq = Some(value);
        self
    }

    pub fn price_ne(mut self, value: f64) -> Self {
        self.price_ne = Some(value);
        self
    }

    pub fn alternative_price_lte(mut self, value: f64) -> Self {
        self.alternative_price_lte = Some(value);
        self
    }

    pub fn alternative_price_gte(mut self, value: f64) -> Self {
        self.alternative_price_gte = Some(value);
        self
    }

    pub fn alternative_price_lt(mut self, value: f64) -> Self {
        self.alternative_price_lt = Some(value);
        self
    }

    pub fn alternative_price_gt(mut self, value: f64) -> Self {
        self.alternative_price_gt = Some(value);
        self
    }

    pub fn alternative_price_eq(mut self, value: f64) -> Self {
        self.alternative_price_eq = Some(value);
        self
    }

    pub fn alternative_price_ne(mut self, value: f64) -> Self {
        self.alternative_price_ne = Some(value);
        self
    }

    pub fn categories(mut self, value: Vec<Option<String>>) -> Self {
        self.categories = Some(value);
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

    pub fn sort_by_field(mut self, value: GetProductsRequestSortByField) -> Self {
        self.sort_by_field = Some(value);
        self
    }

    pub fn is_deleted(mut self, value: impl Into<String>) -> Self {
        self.is_deleted = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetProductsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ids`](GetProductsQueryRequestBuilder::ids)
    /// - [`categories`](GetProductsQueryRequestBuilder::categories)
    pub fn build(self) -> Result<GetProductsQueryRequest, BuildError> {
        Ok(GetProductsQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
            search: self.search,
            name: self.name,
            price_lte: self.price_lte,
            price_gte: self.price_gte,
            price_lt: self.price_lt,
            price_gt: self.price_gt,
            price_eq: self.price_eq,
            price_ne: self.price_ne,
            alternative_price_lte: self.alternative_price_lte,
            alternative_price_gte: self.alternative_price_gte,
            alternative_price_lt: self.alternative_price_lt,
            alternative_price_gt: self.alternative_price_gt,
            alternative_price_eq: self.alternative_price_eq,
            alternative_price_ne: self.alternative_price_ne,
            categories: self
                .categories
                .ok_or_else(|| BuildError::missing_field("categories"))?,
            modified_since: self.modified_since,
            created_since: self.created_since,
            sort_by_field: self.sort_by_field,
            is_deleted: self.is_deleted,
        })
    }
}
