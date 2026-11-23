pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExternalFeedRequest {
    /// Name of the feed
    #[serde(default)]
    pub name: String,
    /// URL of the external data source
    #[serde(default)]
    pub url: String,
    /// Authentication type for accessing the feed
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<CreateExternalFeedRequestAuthType>,
    /// Username for basic authentication (required if authType is 'basic')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password for basic authentication (required if authType is 'basic')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Token for token-based authentication (required if authType is 'token')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Maximum number of retry attempts for failed requests
    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// Whether to cache the feed response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
    /// Custom HTTP headers for the feed request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<CreateExternalFeedRequestHeadersItem>>,
}

impl CreateExternalFeedRequest {
    pub fn builder() -> CreateExternalFeedRequestBuilder {
        <CreateExternalFeedRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExternalFeedRequestBuilder {
    name: Option<String>,
    url: Option<String>,
    auth_type: Option<CreateExternalFeedRequestAuthType>,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
    max_retries: Option<i64>,
    cache: Option<bool>,
    headers: Option<Vec<CreateExternalFeedRequestHeadersItem>>,
}

impl CreateExternalFeedRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn auth_type(mut self, value: CreateExternalFeedRequestAuthType) -> Self {
        self.auth_type = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn max_retries(mut self, value: i64) -> Self {
        self.max_retries = Some(value);
        self
    }

    pub fn cache(mut self, value: bool) -> Self {
        self.cache = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<CreateExternalFeedRequestHeadersItem>) -> Self {
        self.headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateExternalFeedRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateExternalFeedRequestBuilder::name)
    /// - [`url`](CreateExternalFeedRequestBuilder::url)
    pub fn build(self) -> Result<CreateExternalFeedRequest, BuildError> {
        Ok(CreateExternalFeedRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            auth_type: self.auth_type,
            username: self.username,
            password: self.password,
            token: self.token,
            max_retries: self.max_retries,
            cache: self.cache,
            headers: self.headers,
        })
    }
}
