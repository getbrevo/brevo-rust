pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateBatchProductsResponse {
    /// Number of the new created products
    #[serde(rename = "createdCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_count: Option<i64>,
    /// Number of the existing products updated
    #[serde(rename = "updatedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_count: Option<i64>,
}

impl CreateUpdateBatchProductsResponse {
    pub fn builder() -> CreateUpdateBatchProductsResponseBuilder {
        <CreateUpdateBatchProductsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateBatchProductsResponseBuilder {
    created_count: Option<i64>,
    updated_count: Option<i64>,
}

impl CreateUpdateBatchProductsResponseBuilder {
    pub fn created_count(mut self, value: i64) -> Self {
        self.created_count = Some(value);
        self
    }

    pub fn updated_count(mut self, value: i64) -> Self {
        self.updated_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateBatchProductsResponse`].
    pub fn build(self) -> Result<CreateUpdateBatchProductsResponse, BuildError> {
        Ok(CreateUpdateBatchProductsResponse {
            created_count: self.created_count,
            updated_count: self.updated_count,
        })
    }
}
