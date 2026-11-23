pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExternalFeedRequest {
    /// Name of the feed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the external data source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Authentication type for accessing the feed
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<UpdateExternalFeedRequestAuthType>,
    /// Username for basic authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password for basic authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Token for token-based authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Custom HTTP headers for the feed request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<UpdateExternalFeedRequestHeadersItem>>,
    /// Maximum number of retry attempts for failed requests
    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// Whether to cache the feed response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
}

impl UpdateExternalFeedRequest {
    pub fn builder() -> UpdateExternalFeedRequestBuilder {
        <UpdateExternalFeedRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExternalFeedRequestBuilder {
    name: Option<String>,
    url: Option<String>,
    auth_type: Option<UpdateExternalFeedRequestAuthType>,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
    headers: Option<Vec<UpdateExternalFeedRequestHeadersItem>>,
    max_retries: Option<i64>,
    cache: Option<bool>,
}

impl UpdateExternalFeedRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn auth_type(mut self, value: UpdateExternalFeedRequestAuthType) -> Self {
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

    pub fn headers(mut self, value: Vec<UpdateExternalFeedRequestHeadersItem>) -> Self {
        self.headers = Some(value);
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

    /// Consumes the builder and constructs a [`UpdateExternalFeedRequest`].
    pub fn build(self) -> Result<UpdateExternalFeedRequest, BuildError> {
        Ok(UpdateExternalFeedRequest {
            name: self.name,
            url: self.url,
            auth_type: self.auth_type,
            username: self.username,
            password: self.password,
            token: self.token,
            headers: self.headers,
            max_retries: self.max_retries,
            cache: self.cache,
        })
    }
}
