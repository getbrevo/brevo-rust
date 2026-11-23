pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletPassInstallUrl {
    /// The wallet installation URL for the contact. Opening it lets the contact add the pass to their Apple Wallet or Google Wallet.
    #[serde(default)]
    pub url: String,
}

impl WalletPassInstallUrl {
    pub fn builder() -> WalletPassInstallUrlBuilder {
        <WalletPassInstallUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletPassInstallUrlBuilder {
    url: Option<String>,
}

impl WalletPassInstallUrlBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WalletPassInstallUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](WalletPassInstallUrlBuilder::url)
    pub fn build(self) -> Result<WalletPassInstallUrl, BuildError> {
        Ok(WalletPassInstallUrl {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
