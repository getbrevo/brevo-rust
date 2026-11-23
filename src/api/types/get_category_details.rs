pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCategoryDetails {
    /// Creation UTC date-time of the category (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Category ID for which you requested the details
    #[serde(default)]
    pub id: String,
    /// category deleted from the shop's database
    #[serde(rename = "isDeleted")]
    #[serde(default)]
    pub is_deleted: bool,
    /// Last modification UTC date-time of the category (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Name of the category for which you requested the details
    #[serde(default)]
    pub name: String,
    /// URL to the category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GetCategoryDetails {
    pub fn builder() -> GetCategoryDetailsBuilder {
        <GetCategoryDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCategoryDetailsBuilder {
    created_at: Option<String>,
    id: Option<String>,
    is_deleted: Option<bool>,
    modified_at: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

impl GetCategoryDetailsBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetCategoryDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetCategoryDetailsBuilder::created_at)
    /// - [`id`](GetCategoryDetailsBuilder::id)
    /// - [`is_deleted`](GetCategoryDetailsBuilder::is_deleted)
    /// - [`modified_at`](GetCategoryDetailsBuilder::modified_at)
    /// - [`name`](GetCategoryDetailsBuilder::name)
    pub fn build(self) -> Result<GetCategoryDetails, BuildError> {
        Ok(GetCategoryDetails {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_deleted: self
                .is_deleted
                .ok_or_else(|| BuildError::missing_field("is_deleted"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url,
        })
    }
}
