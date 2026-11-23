pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateUpdateProductResponse {
    /// ID of the product when a new product is created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateUpdateProductResponse {
    pub fn builder() -> CreateUpdateProductResponseBuilder {
        <CreateUpdateProductResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateProductResponseBuilder {
    id: Option<String>,
}

impl CreateUpdateProductResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateProductResponse`].
    pub fn build(self) -> Result<CreateUpdateProductResponse, BuildError> {
        Ok(CreateUpdateProductResponse { id: self.id })
    }
}
