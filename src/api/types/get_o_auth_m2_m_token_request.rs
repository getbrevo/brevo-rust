pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetOAuthM2MTokenRequest {
    pub grant_type: String,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub client_secret: String,
    /// Space-separated list of requested scopes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl GetOAuthM2MTokenRequest {
    pub fn builder() -> GetOAuthM2MTokenRequestBuilder {
        <GetOAuthM2MTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOAuthM2MTokenRequestBuilder {
    grant_type: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    scope: Option<String>,
}

impl GetOAuthM2MTokenRequestBuilder {
    pub fn grant_type(mut self, value: impl Into<String>) -> Self {
        self.grant_type = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOAuthM2MTokenRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`grant_type`](GetOAuthM2MTokenRequestBuilder::grant_type)
    /// - [`client_id`](GetOAuthM2MTokenRequestBuilder::client_id)
    /// - [`client_secret`](GetOAuthM2MTokenRequestBuilder::client_secret)
    pub fn build(self) -> Result<GetOAuthM2MTokenRequest, BuildError> {
        Ok(GetOAuthM2MTokenRequest {
            grant_type: self
                .grant_type
                .ok_or_else(|| BuildError::missing_field("grant_type"))?,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_secret: self
                .client_secret
                .ok_or_else(|| BuildError::missing_field("client_secret"))?,
            scope: self.scope,
        })
    }
}
