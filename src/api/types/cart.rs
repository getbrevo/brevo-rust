pub use crate::prelude::*;

/// Specify the payment currency and amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Cart {
    /// Currency code for the payment amount.
    pub currency: String,
    /// Payment amount, in cents. e.g. if you want to request €12.00, then the amount in cents is 1200.
    #[serde(rename = "specificAmount")]
    #[serde(default)]
    pub specific_amount: i64,
}

impl Cart {
    pub fn builder() -> CartBuilder {
        <CartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CartBuilder {
    currency: Option<String>,
    specific_amount: Option<i64>,
}

impl CartBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn specific_amount(mut self, value: i64) -> Self {
        self.specific_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Cart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CartBuilder::currency)
    /// - [`specific_amount`](CartBuilder::specific_amount)
    pub fn build(self) -> Result<Cart, BuildError> {
        Ok(Cart {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            specific_amount: self
                .specific_amount
                .ok_or_else(|| BuildError::missing_field("specific_amount"))?,
        })
    }
}
