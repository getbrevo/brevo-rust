pub use crate::prelude::*;

/// Information about your transactional email account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountResponseRelay {
    /// Data regarding the transactional email account
    #[serde(default)]
    pub data: GetAccountResponseRelayData,
    /// Status of your transactional email Account
    /// (true=Enabled, false=Disabled)
    #[serde(default)]
    pub enabled: bool,
}

impl GetAccountResponseRelay {
    pub fn builder() -> GetAccountResponseRelayBuilder {
        <GetAccountResponseRelayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseRelayBuilder {
    data: Option<GetAccountResponseRelayData>,
    enabled: Option<bool>,
}

impl GetAccountResponseRelayBuilder {
    pub fn data(mut self, value: GetAccountResponseRelayData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponseRelay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](GetAccountResponseRelayBuilder::data)
    /// - [`enabled`](GetAccountResponseRelayBuilder::enabled)
    pub fn build(self) -> Result<GetAccountResponseRelay, BuildError> {
        Ok(GetAccountResponseRelay {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
