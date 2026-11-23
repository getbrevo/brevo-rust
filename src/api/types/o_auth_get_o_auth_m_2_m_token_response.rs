pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOAuthM2MTokenResponse {
    #[serde(default)]
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    /// Token lifetime in seconds (~3600).
    #[serde(default)]
    pub expires_in: i64,
}

impl GetOAuthM2MTokenResponse {
    pub fn builder() -> GetOAuthM2MTokenResponseBuilder {
        <GetOAuthM2MTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOAuthM2MTokenResponseBuilder {
    access_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<i64>,
}

impl GetOAuthM2MTokenResponseBuilder {
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    pub fn token_type(mut self, value: impl Into<String>) -> Self {
        self.token_type = Some(value.into());
        self
    }

    pub fn expires_in(mut self, value: i64) -> Self {
        self.expires_in = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOAuthM2MTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_token`](GetOAuthM2MTokenResponseBuilder::access_token)
    /// - [`expires_in`](GetOAuthM2MTokenResponseBuilder::expires_in)
    pub fn build(self) -> Result<GetOAuthM2MTokenResponse, BuildError> {
        Ok(GetOAuthM2MTokenResponse {
            access_token: self
                .access_token
                .ok_or_else(|| BuildError::missing_field("access_token"))?,
            token_type: self.token_type,
            expires_in: self
                .expires_in
                .ok_or_else(|| BuildError::missing_field("expires_in"))?,
        })
    }
}
