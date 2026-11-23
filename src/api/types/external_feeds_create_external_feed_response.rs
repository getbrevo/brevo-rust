pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExternalFeedResponse {
    /// UUID of the created feed
    #[serde(default)]
    pub id: String,
}

impl CreateExternalFeedResponse {
    pub fn builder() -> CreateExternalFeedResponseBuilder {
        <CreateExternalFeedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExternalFeedResponseBuilder {
    id: Option<String>,
}

impl CreateExternalFeedResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateExternalFeedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateExternalFeedResponseBuilder::id)
    pub fn build(self) -> Result<CreateExternalFeedResponse, BuildError> {
        Ok(CreateExternalFeedResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
