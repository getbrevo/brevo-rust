pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletPassRedeemResponse {
    /// Human-readable confirmation message.
    #[serde(default)]
    pub message: String,
    /// Indicates whether the scan/redemption was recorded.
    #[serde(default)]
    pub success: bool,
}

impl WalletPassRedeemResponse {
    pub fn builder() -> WalletPassRedeemResponseBuilder {
        <WalletPassRedeemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletPassRedeemResponseBuilder {
    message: Option<String>,
    success: Option<bool>,
}

impl WalletPassRedeemResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WalletPassRedeemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](WalletPassRedeemResponseBuilder::message)
    /// - [`success`](WalletPassRedeemResponseBuilder::success)
    pub fn build(self) -> Result<WalletPassRedeemResponse, BuildError> {
        Ok(WalletPassRedeemResponse {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
