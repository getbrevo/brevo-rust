pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetExternalFeedByUuidResponse {
    /// Unique identifier of the feed
    #[serde(default)]
    pub id: String,
    /// Name of the feed
    #[serde(default)]
    pub name: String,
    /// URL of the external data source
    #[serde(default)]
    pub url: String,
    /// Authentication type for accessing the feed
    #[serde(rename = "authType")]
    pub auth_type: GetExternalFeedByUuidResponseAuthType,
    /// Username for basic authentication. Only returned when authType is 'basic'. Excluded from response when authType is 'token'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password for basic authentication. Only returned when authType is 'basic'. Excluded from response when authType is 'token'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Token for token-based authentication. Only returned when authType is 'token'. Excluded from response when authType is 'basic' or 'noAuth'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Custom HTTP headers for the feed request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<GetExternalFeedByUuidResponseHeadersItem>>,
    /// Maximum number of retry attempts for failed requests
    #[serde(rename = "maxRetries")]
    #[serde(default)]
    pub max_retries: i64,
    /// Whether to cache the feed response
    #[serde(default)]
    pub cache: bool,
    /// Feed creation timestamp
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Feed last modification timestamp
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub modified_at: DateTime<FixedOffset>,
}

impl GetExternalFeedByUuidResponse {
    pub fn builder() -> GetExternalFeedByUuidResponseBuilder {
        <GetExternalFeedByUuidResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetExternalFeedByUuidResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
    auth_type: Option<GetExternalFeedByUuidResponseAuthType>,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
    headers: Option<Vec<GetExternalFeedByUuidResponseHeadersItem>>,
    max_retries: Option<i64>,
    cache: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    modified_at: Option<DateTime<FixedOffset>>,
}

impl GetExternalFeedByUuidResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn auth_type(mut self, value: GetExternalFeedByUuidResponseAuthType) -> Self {
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

    pub fn headers(mut self, value: Vec<GetExternalFeedByUuidResponseHeadersItem>) -> Self {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetExternalFeedByUuidResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetExternalFeedByUuidResponseBuilder::id)
    /// - [`name`](GetExternalFeedByUuidResponseBuilder::name)
    /// - [`url`](GetExternalFeedByUuidResponseBuilder::url)
    /// - [`auth_type`](GetExternalFeedByUuidResponseBuilder::auth_type)
    /// - [`max_retries`](GetExternalFeedByUuidResponseBuilder::max_retries)
    /// - [`cache`](GetExternalFeedByUuidResponseBuilder::cache)
    /// - [`created_at`](GetExternalFeedByUuidResponseBuilder::created_at)
    /// - [`modified_at`](GetExternalFeedByUuidResponseBuilder::modified_at)
    pub fn build(self) -> Result<GetExternalFeedByUuidResponse, BuildError> {
        Ok(GetExternalFeedByUuidResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            auth_type: self
                .auth_type
                .ok_or_else(|| BuildError::missing_field("auth_type"))?,
            username: self.username,
            password: self.password,
            token: self.token,
            headers: self.headers,
            max_retries: self
                .max_retries
                .ok_or_else(|| BuildError::missing_field("max_retries"))?,
            cache: self
                .cache
                .ok_or_else(|| BuildError::missing_field("cache"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
        })
    }
}
