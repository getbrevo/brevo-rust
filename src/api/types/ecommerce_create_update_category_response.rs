pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateCategoryResponse {
    /// ID of the category when a new category is created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateUpdateCategoryResponse {
    pub fn builder() -> CreateUpdateCategoryResponseBuilder {
        <CreateUpdateCategoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateCategoryResponseBuilder {
    id: Option<String>,
}

impl CreateUpdateCategoryResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateCategoryResponse`].
    pub fn build(self) -> Result<CreateUpdateCategoryResponse, BuildError> {
        Ok(CreateUpdateCategoryResponse { id: self.id })
    }
}
