pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCouponCollectionResponse {
    /// The id of the created collection
    #[serde(default)]
    pub id: String,
}

impl CreateCouponCollectionResponse {
    pub fn builder() -> CreateCouponCollectionResponseBuilder {
        <CreateCouponCollectionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCouponCollectionResponseBuilder {
    id: Option<String>,
}

impl CreateCouponCollectionResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCouponCollectionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateCouponCollectionResponseBuilder::id)
    pub fn build(self) -> Result<CreateCouponCollectionResponse, BuildError> {
        Ok(CreateCouponCollectionResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
