pub use crate::prelude::*;

/// Identifies the contact associated with the order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrderIdentifiers {
    /// Email of the contact associated with the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// ext_id associated with the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// loyalty_subscription_id associated with the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Phone number of the contact associated with the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_id: Option<String>,
}

impl OrderIdentifiers {
    pub fn builder() -> OrderIdentifiersBuilder {
        <OrderIdentifiersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderIdentifiersBuilder {
    email_id: Option<String>,
    ext_id: Option<String>,
    loyalty_subscription_id: Option<String>,
    phone_id: Option<String>,
}

impl OrderIdentifiersBuilder {
    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn phone_id(mut self, value: impl Into<String>) -> Self {
        self.phone_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrderIdentifiers`].
    pub fn build(self) -> Result<OrderIdentifiers, BuildError> {
        Ok(OrderIdentifiers {
            email_id: self.email_id,
            ext_id: self.ext_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
            phone_id: self.phone_id,
        })
    }
}
