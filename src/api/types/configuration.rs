pub use crate::prelude::*;

/// Optional. Redirect contact to a custom success page once payment is successful. If empty the default Brevo page will be displayed once a payment is validated
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Configuration {
    /// Absolute URL of the custom success page.
    #[serde(rename = "customSuccessUrl")]
    #[serde(default)]
    pub custom_success_url: String,
}

impl Configuration {
    pub fn builder() -> ConfigurationBuilder {
        <ConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigurationBuilder {
    custom_success_url: Option<String>,
}

impl ConfigurationBuilder {
    pub fn custom_success_url(mut self, value: impl Into<String>) -> Self {
        self.custom_success_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Configuration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_success_url`](ConfigurationBuilder::custom_success_url)
    pub fn build(self) -> Result<Configuration, BuildError> {
        Ok(Configuration {
            custom_success_url: self
                .custom_success_url
                .ok_or_else(|| BuildError::missing_field("custom_success_url"))?,
        })
    }
}
