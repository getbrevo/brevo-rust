pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetWebhook {
    /// Authentication header to be send with the webhook requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<HashMap<String, serde_json::Value>>,
    /// Batching configuration of the webhook, we send batched webhooks if its
    /// true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batched: Option<bool>,
    /// Creation UTC date-time of the webhook (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Description of the webhook
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// ID of the webhook
    #[serde(default)]
    pub id: i64,
    /// Last modification UTC date-time of the webhook (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Type of webhook (marketing, transactional, or inbound)
    pub r#type: GetWebhookType,
    /// Inbound domain of the webhook, only returned for inbound type webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// URL of the webhook
    #[serde(default)]
    pub url: String,
}

impl GetWebhook {
    pub fn builder() -> GetWebhookBuilder {
        <GetWebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWebhookBuilder {
    auth: Option<HashMap<String, serde_json::Value>>,
    batched: Option<bool>,
    created_at: Option<String>,
    description: Option<String>,
    events: Option<Vec<String>>,
    headers: Option<Vec<HashMap<String, serde_json::Value>>>,
    id: Option<i64>,
    modified_at: Option<String>,
    r#type: Option<GetWebhookType>,
    domain: Option<String>,
    url: Option<String>,
}

impl GetWebhookBuilder {
    pub fn auth(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn batched(mut self, value: bool) -> Self {
        self.batched = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: GetWebhookType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWebhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetWebhookBuilder::created_at)
    /// - [`description`](GetWebhookBuilder::description)
    /// - [`events`](GetWebhookBuilder::events)
    /// - [`id`](GetWebhookBuilder::id)
    /// - [`modified_at`](GetWebhookBuilder::modified_at)
    /// - [`r#type`](GetWebhookBuilder::r#type)
    /// - [`url`](GetWebhookBuilder::url)
    pub fn build(self) -> Result<GetWebhook, BuildError> {
        Ok(GetWebhook {
            auth: self.auth,
            batched: self.batched,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            headers: self.headers,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            domain: self.domain,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
