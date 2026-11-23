pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhookRequest {
    /// Add authentication on webhook url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<UpdateWebhookRequestAuth>,
    /// To send batched webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batched: Option<bool>,
    /// Description of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inbound domain of webhook, used in case of event type `inbound`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// - Events triggering the webhook. Possible values for
    /// **Transactional** type webhook: #### `sent` OR `request`,
    /// `delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`,
    /// `invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and
    /// `unsubscribed` - Possible values for **Marketing** type webhook:
    /// #### `spam`, `opened`, `click`, `hardBounce`, `softBounce`,
    /// `unsubscribed`, `listAddition` & `delivered` - Possible values
    /// for **Inbound** type webhook: #### `inboundEmailProcessed`
    /// #### `reply`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<UpdateWebhookRequestEventsItem>>,
    /// Custom headers to be send with webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<UpdateWebhookRequestHeadersItem>>,
    /// URL of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdateWebhookRequest {
    pub fn builder() -> UpdateWebhookRequestBuilder {
        <UpdateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhookRequestBuilder {
    auth: Option<UpdateWebhookRequestAuth>,
    batched: Option<bool>,
    description: Option<String>,
    domain: Option<String>,
    events: Option<Vec<UpdateWebhookRequestEventsItem>>,
    headers: Option<Vec<UpdateWebhookRequestHeadersItem>>,
    url: Option<String>,
}

impl UpdateWebhookRequestBuilder {
    pub fn auth(mut self, value: UpdateWebhookRequestAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn batched(mut self, value: bool) -> Self {
        self.batched = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<UpdateWebhookRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<UpdateWebhookRequestHeadersItem>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhookRequest`].
    pub fn build(self) -> Result<UpdateWebhookRequest, BuildError> {
        Ok(UpdateWebhookRequest {
            auth: self.auth,
            batched: self.batched,
            description: self.description,
            domain: self.domain,
            events: self.events,
            headers: self.headers,
            url: self.url,
        })
    }
}
