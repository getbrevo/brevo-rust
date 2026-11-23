pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCategoriesResponse {
    #[serde(default)]
    pub categories: Vec<GetCategoryDetails>,
    /// Number of categories
    #[serde(default)]
    pub count: i64,
}

impl GetCategoriesResponse {
    pub fn builder() -> GetCategoriesResponseBuilder {
        <GetCategoriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCategoriesResponseBuilder {
    categories: Option<Vec<GetCategoryDetails>>,
    count: Option<i64>,
}

impl GetCategoriesResponseBuilder {
    pub fn categories(mut self, value: Vec<GetCategoryDetails>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCategoriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`categories`](GetCategoriesResponseBuilder::categories)
    /// - [`count`](GetCategoriesResponseBuilder::count)
    pub fn build(self) -> Result<GetCategoriesResponse, BuildError> {
        Ok(GetCategoriesResponse {
            categories: self
                .categories
                .ok_or_else(|| BuildError::missing_field("categories"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
        })
    }
}
