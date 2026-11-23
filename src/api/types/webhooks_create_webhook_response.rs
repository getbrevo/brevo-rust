pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateWebhookResponse {
    pub fn builder() -> CreateWebhookResponseBuilder {
        <CreateWebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookResponseBuilder {
    id: Option<i64>,
}

impl CreateWebhookResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateWebhookResponseBuilder::id)
    pub fn build(self) -> Result<CreateWebhookResponse, BuildError> {
        Ok(CreateWebhookResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
