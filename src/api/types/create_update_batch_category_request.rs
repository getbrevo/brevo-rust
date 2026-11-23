pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateBatchCategoryRequest {
    /// array of categories objects
    #[serde(default)]
    pub categories: Vec<CreateUpdateBatchCategoryRequestCategoriesItem>,
    /// Facilitate to update the existing categories in the same request (updateEnabled = true)
    #[serde(rename = "updateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_enabled: Option<bool>,
}

impl CreateUpdateBatchCategoryRequest {
    pub fn builder() -> CreateUpdateBatchCategoryRequestBuilder {
        <CreateUpdateBatchCategoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateBatchCategoryRequestBuilder {
    categories: Option<Vec<CreateUpdateBatchCategoryRequestCategoriesItem>>,
    update_enabled: Option<bool>,
}

impl CreateUpdateBatchCategoryRequestBuilder {
    pub fn categories(
        mut self,
        value: Vec<CreateUpdateBatchCategoryRequestCategoriesItem>,
    ) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn update_enabled(mut self, value: bool) -> Self {
        self.update_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateBatchCategoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`categories`](CreateUpdateBatchCategoryRequestBuilder::categories)
    pub fn build(self) -> Result<CreateUpdateBatchCategoryRequest, BuildError> {
        Ok(CreateUpdateBatchCategoryRequest {
            categories: self
                .categories
                .ok_or_else(|| BuildError::missing_field("categories"))?,
            update_enabled: self.update_enabled,
        })
    }
}
