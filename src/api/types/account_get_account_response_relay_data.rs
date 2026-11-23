pub use crate::prelude::*;

/// Data regarding the transactional email account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountResponseRelayData {
    /// Port used for SMTP Relay
    #[serde(default)]
    pub port: i64,
    /// URL of the SMTP Relay
    #[serde(default)]
    pub relay: String,
    /// Email to use as login on transactional platform
    #[serde(rename = "userName")]
    #[serde(default)]
    pub user_name: String,
}

impl GetAccountResponseRelayData {
    pub fn builder() -> GetAccountResponseRelayDataBuilder {
        <GetAccountResponseRelayDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseRelayDataBuilder {
    port: Option<i64>,
    relay: Option<String>,
    user_name: Option<String>,
}

impl GetAccountResponseRelayDataBuilder {
    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    pub fn relay(mut self, value: impl Into<String>) -> Self {
        self.relay = Some(value.into());
        self
    }

    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponseRelayData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`port`](GetAccountResponseRelayDataBuilder::port)
    /// - [`relay`](GetAccountResponseRelayDataBuilder::relay)
    /// - [`user_name`](GetAccountResponseRelayDataBuilder::user_name)
    pub fn build(self) -> Result<GetAccountResponseRelayData, BuildError> {
        Ok(GetAccountResponseRelayData {
            port: self.port.ok_or_else(|| BuildError::missing_field("port"))?,
            relay: self
                .relay
                .ok_or_else(|| BuildError::missing_field("relay"))?,
            user_name: self
                .user_name
                .ok_or_else(|| BuildError::missing_field("user_name"))?,
        })
    }
}
