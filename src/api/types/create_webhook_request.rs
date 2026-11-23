pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookRequest {
    /// Add authentication on webhook url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<CreateWebhookRequestAuth>,
    /// To send batched webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batched: Option<bool>,
    /// Channel of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<CreateWebhookRequestChannel>,
    /// Description of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inbound domain of webhook, required in case of event type
    /// `inbound`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Events triggering the webhook. Required for transactional and
    /// marketing types, optional for inbound type (defaults to
    /// `inboundEmailProcessed`). Possible values for **Transactional**
    /// type webhook: `sent` OR `request`, `delivered`, `hardBounce`,
    /// `softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`,
    /// `opened`, `uniqueOpened` and `unsubscribed`. Possible values for
    /// **Marketing** type webhook: `spam`, `opened`, `click`,
    /// `hardBounce`, `softBounce`, `unsubscribed`, `listAddition`,
    /// `delivered`, `contactUpdated` & `contactDeleted`. Possible values
    /// for **Inbound** type webhook: `inboundEmailProcessed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<CreateWebhookRequestEventsItem>>,
    /// Custom headers to be send with webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<CreateWebhookRequestHeadersItem>>,
    /// Type of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateWebhookRequestType>,
    /// URL of the webhook
    #[serde(default)]
    pub url: String,
}

impl CreateWebhookRequest {
    pub fn builder() -> CreateWebhookRequestBuilder {
        <CreateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookRequestBuilder {
    auth: Option<CreateWebhookRequestAuth>,
    batched: Option<bool>,
    channel: Option<CreateWebhookRequestChannel>,
    description: Option<String>,
    domain: Option<String>,
    events: Option<Vec<CreateWebhookRequestEventsItem>>,
    headers: Option<Vec<CreateWebhookRequestHeadersItem>>,
    r#type: Option<CreateWebhookRequestType>,
    url: Option<String>,
}

impl CreateWebhookRequestBuilder {
    pub fn auth(mut self, value: CreateWebhookRequestAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn batched(mut self, value: bool) -> Self {
        self.batched = Some(value);
        self
    }

    pub fn channel(mut self, value: CreateWebhookRequestChannel) -> Self {
        self.channel = Some(value);
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

    pub fn events(mut self, value: Vec<CreateWebhookRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<CreateWebhookRequestHeadersItem>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateWebhookRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateWebhookRequestBuilder::url)
    pub fn build(self) -> Result<CreateWebhookRequest, BuildError> {
        Ok(CreateWebhookRequest {
            auth: self.auth,
            batched: self.batched,
            channel: self.channel,
            description: self.description,
            domain: self.domain,
            events: self.events,
            headers: self.headers,
            r#type: self.r#type,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
