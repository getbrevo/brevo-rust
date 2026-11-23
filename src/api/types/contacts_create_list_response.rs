pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateListResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateListResponse {
    pub fn builder() -> CreateListResponseBuilder {
        <CreateListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateListResponseBuilder {
    id: Option<i64>,
}

impl CreateListResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateListResponseBuilder::id)
    pub fn build(self) -> Result<CreateListResponse, BuildError> {
        Ok(CreateListResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
