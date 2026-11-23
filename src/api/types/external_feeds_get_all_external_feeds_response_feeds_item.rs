pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAllExternalFeedsResponseFeedsItem {
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
    pub auth_type: GetAllExternalFeedsResponseFeedsItemAuthType,
    /// Username for basic authentication. Only returned when authType is 'basic'. Excluded when authType is 'token'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password for basic authentication. Only returned when authType is 'basic'. Excluded when authType is 'token'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Token for token-based authentication. Only returned when authType is 'token'. Excluded when authType is 'basic' or 'noAuth'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Maximum number of retry attempts for failed requests
    #[serde(rename = "maxRetries")]
    #[serde(default)]
    pub max_retries: i64,
    /// Whether to cache the feed response
    #[serde(default)]
    pub cache: bool,
    /// Custom HTTP headers for the feed request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<GetAllExternalFeedsResponseFeedsItemHeadersItem>>,
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

impl GetAllExternalFeedsResponseFeedsItem {
    pub fn builder() -> GetAllExternalFeedsResponseFeedsItemBuilder {
        <GetAllExternalFeedsResponseFeedsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllExternalFeedsResponseFeedsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
    auth_type: Option<GetAllExternalFeedsResponseFeedsItemAuthType>,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
    max_retries: Option<i64>,
    cache: Option<bool>,
    headers: Option<Vec<GetAllExternalFeedsResponseFeedsItemHeadersItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    modified_at: Option<DateTime<FixedOffset>>,
}

impl GetAllExternalFeedsResponseFeedsItemBuilder {
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

    pub fn auth_type(mut self, value: GetAllExternalFeedsResponseFeedsItemAuthType) -> Self {
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

    pub fn headers(mut self, value: Vec<GetAllExternalFeedsResponseFeedsItemHeadersItem>) -> Self {
        self.headers = Some(value);
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

    /// Consumes the builder and constructs a [`GetAllExternalFeedsResponseFeedsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetAllExternalFeedsResponseFeedsItemBuilder::id)
    /// - [`name`](GetAllExternalFeedsResponseFeedsItemBuilder::name)
    /// - [`url`](GetAllExternalFeedsResponseFeedsItemBuilder::url)
    /// - [`auth_type`](GetAllExternalFeedsResponseFeedsItemBuilder::auth_type)
    /// - [`max_retries`](GetAllExternalFeedsResponseFeedsItemBuilder::max_retries)
    /// - [`cache`](GetAllExternalFeedsResponseFeedsItemBuilder::cache)
    /// - [`created_at`](GetAllExternalFeedsResponseFeedsItemBuilder::created_at)
    /// - [`modified_at`](GetAllExternalFeedsResponseFeedsItemBuilder::modified_at)
    pub fn build(self) -> Result<GetAllExternalFeedsResponseFeedsItem, BuildError> {
        Ok(GetAllExternalFeedsResponseFeedsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            auth_type: self
                .auth_type
                .ok_or_else(|| BuildError::missing_field("auth_type"))?,
            username: self.username,
            password: self.password,
            token: self.token,
            max_retries: self
                .max_retries
                .ok_or_else(|| BuildError::missing_field("max_retries"))?,
            cache: self
                .cache
                .ok_or_else(|| BuildError::missing_field("cache"))?,
            headers: self.headers,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
        })
    }
}
