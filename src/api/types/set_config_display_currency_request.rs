pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetConfigDisplayCurrencyRequest {
    /// ISO 4217 compliant display currency code
    #[serde(default)]
    pub code: String,
}

impl SetConfigDisplayCurrencyRequest {
    pub fn builder() -> SetConfigDisplayCurrencyRequestBuilder {
        <SetConfigDisplayCurrencyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetConfigDisplayCurrencyRequestBuilder {
    code: Option<String>,
}

impl SetConfigDisplayCurrencyRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetConfigDisplayCurrencyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SetConfigDisplayCurrencyRequestBuilder::code)
    pub fn build(self) -> Result<SetConfigDisplayCurrencyRequest, BuildError> {
        Ok(SetConfigDisplayCurrencyRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
