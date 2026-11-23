pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetWebhooksResponse {
    #[serde(default)]
    pub webhooks: Vec<GetWebhook>,
}

impl GetWebhooksResponse {
    pub fn builder() -> GetWebhooksResponseBuilder {
        <GetWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWebhooksResponseBuilder {
    webhooks: Option<Vec<GetWebhook>>,
}

impl GetWebhooksResponseBuilder {
    pub fn webhooks(mut self, value: Vec<GetWebhook>) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhooks`](GetWebhooksResponseBuilder::webhooks)
    pub fn build(self) -> Result<GetWebhooksResponse, BuildError> {
        Ok(GetWebhooksResponse {
            webhooks: self
                .webhooks
                .ok_or_else(|| BuildError::missing_field("webhooks"))?,
        })
    }
}
