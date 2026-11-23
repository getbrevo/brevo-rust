pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountResponseMarketingAutomation {
    /// Status of Marketing Automation Plateform activation
    /// for your account (true=enabled, false=disabled)
    #[serde(default)]
    pub enabled: bool,
    /// Marketing Automation Tracker ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl GetAccountResponseMarketingAutomation {
    pub fn builder() -> GetAccountResponseMarketingAutomationBuilder {
        <GetAccountResponseMarketingAutomationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseMarketingAutomationBuilder {
    enabled: Option<bool>,
    key: Option<String>,
}

impl GetAccountResponseMarketingAutomationBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponseMarketingAutomation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](GetAccountResponseMarketingAutomationBuilder::enabled)
    pub fn build(self) -> Result<GetAccountResponseMarketingAutomation, BuildError> {
        Ok(GetAccountResponseMarketingAutomation {
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            key: self.key,
        })
    }
}
