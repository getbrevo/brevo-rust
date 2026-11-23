pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEcommerceConfigDisplayCurrencyResponse {
    /// ISO 4217 compliant display currency code
    #[serde(default)]
    pub code: String,
}

impl GetEcommerceConfigDisplayCurrencyResponse {
    pub fn builder() -> GetEcommerceConfigDisplayCurrencyResponseBuilder {
        <GetEcommerceConfigDisplayCurrencyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEcommerceConfigDisplayCurrencyResponseBuilder {
    code: Option<String>,
}

impl GetEcommerceConfigDisplayCurrencyResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetEcommerceConfigDisplayCurrencyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GetEcommerceConfigDisplayCurrencyResponseBuilder::code)
    pub fn build(self) -> Result<GetEcommerceConfigDisplayCurrencyResponse, BuildError> {
        Ok(GetEcommerceConfigDisplayCurrencyResponse {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
