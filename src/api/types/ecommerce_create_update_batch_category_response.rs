pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateBatchCategoryResponse {
    /// Number of the new created categories
    #[serde(rename = "createdCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_count: Option<i64>,
    /// Number of the existing categories updated
    #[serde(rename = "updatedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_count: Option<i64>,
}

impl CreateUpdateBatchCategoryResponse {
    pub fn builder() -> CreateUpdateBatchCategoryResponseBuilder {
        <CreateUpdateBatchCategoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateBatchCategoryResponseBuilder {
    created_count: Option<i64>,
    updated_count: Option<i64>,
}

impl CreateUpdateBatchCategoryResponseBuilder {
    pub fn created_count(mut self, value: i64) -> Self {
        self.created_count = Some(value);
        self
    }

    pub fn updated_count(mut self, value: i64) -> Self {
        self.updated_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateBatchCategoryResponse`].
    pub fn build(self) -> Result<CreateUpdateBatchCategoryResponse, BuildError> {
        Ok(CreateUpdateBatchCategoryResponse {
            created_count: self.created_count,
            updated_count: self.updated_count,
        })
    }
}
