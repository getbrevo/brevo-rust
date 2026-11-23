pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateBatchCategoryRequestCategoriesItem {
    /// UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the category deleted from the shop's database
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Unique Category ID as saved in the shop
    #[serde(default)]
    pub id: String,
    /// category deleted from the shop's database
    #[serde(rename = "isDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// **Mandatory in case of creation**. Name of the Category, as displayed in the shop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL to the category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreateUpdateBatchCategoryRequestCategoriesItem {
    pub fn builder() -> CreateUpdateBatchCategoryRequestCategoriesItemBuilder {
        <CreateUpdateBatchCategoryRequestCategoriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateBatchCategoryRequestCategoriesItemBuilder {
    deleted_at: Option<String>,
    id: Option<String>,
    is_deleted: Option<bool>,
    name: Option<String>,
    url: Option<String>,
}

impl CreateUpdateBatchCategoryRequestCategoriesItemBuilder {
    pub fn deleted_at(mut self, value: impl Into<String>) -> Self {
        self.deleted_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_deleted(mut self, value: bool) -> Self {
        self.is_deleted = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateBatchCategoryRequestCategoriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateUpdateBatchCategoryRequestCategoriesItemBuilder::id)
    pub fn build(self) -> Result<CreateUpdateBatchCategoryRequestCategoriesItem, BuildError> {
        Ok(CreateUpdateBatchCategoryRequestCategoriesItem {
            deleted_at: self.deleted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_deleted: self.is_deleted,
            name: self.name,
            url: self.url,
        })
    }
}
