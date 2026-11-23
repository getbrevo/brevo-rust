pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetConfigDisplayCurrencyResponse {
    /// ISO 4217 compliant display currency code
    #[serde(default)]
    pub code: String,
}

impl SetConfigDisplayCurrencyResponse {
    pub fn builder() -> SetConfigDisplayCurrencyResponseBuilder {
        <SetConfigDisplayCurrencyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetConfigDisplayCurrencyResponseBuilder {
    code: Option<String>,
}

impl SetConfigDisplayCurrencyResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetConfigDisplayCurrencyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SetConfigDisplayCurrencyResponseBuilder::code)
    pub fn build(self) -> Result<SetConfigDisplayCurrencyResponse, BuildError> {
        Ok(SetConfigDisplayCurrencyResponse {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
