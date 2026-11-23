pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RedeemVoucherRequestOrderIdentifiers {
    /// External identifier for the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// Identifier for the loyalty subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
}

impl RedeemVoucherRequestOrderIdentifiers {
    pub fn builder() -> RedeemVoucherRequestOrderIdentifiersBuilder {
        <RedeemVoucherRequestOrderIdentifiersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemVoucherRequestOrderIdentifiersBuilder {
    ext_id: Option<String>,
    loyalty_subscription_id: Option<String>,
}

impl RedeemVoucherRequestOrderIdentifiersBuilder {
    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RedeemVoucherRequestOrderIdentifiers`].
    pub fn build(self) -> Result<RedeemVoucherRequestOrderIdentifiers, BuildError> {
        Ok(RedeemVoucherRequestOrderIdentifiers {
            ext_id: self.ext_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
        })
    }
}
