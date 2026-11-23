pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateCategoryRequest {
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
    /// Facilitate to update the existing category in the same request (updateEnabled = true)
    #[serde(rename = "updateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_enabled: Option<bool>,
    /// URL to the category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreateUpdateCategoryRequest {
    pub fn builder() -> CreateUpdateCategoryRequestBuilder {
        <CreateUpdateCategoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateCategoryRequestBuilder {
    deleted_at: Option<String>,
    id: Option<String>,
    is_deleted: Option<bool>,
    name: Option<String>,
    update_enabled: Option<bool>,
    url: Option<String>,
}

impl CreateUpdateCategoryRequestBuilder {
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

    pub fn update_enabled(mut self, value: bool) -> Self {
        self.update_enabled = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateCategoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateUpdateCategoryRequestBuilder::id)
    pub fn build(self) -> Result<CreateUpdateCategoryRequest, BuildError> {
        Ok(CreateUpdateCategoryRequest {
            deleted_at: self.deleted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_deleted: self.is_deleted,
            name: self.name,
            update_enabled: self.update_enabled,
            url: self.url,
        })
    }
}
