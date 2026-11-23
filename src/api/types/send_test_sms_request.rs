pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTestSmsRequest {
    /// Mobile number of the recipient with the country code. This number **must belong to one of your contacts in Brevo account and must not be blacklisted**
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl SendTestSmsRequest {
    pub fn builder() -> SendTestSmsRequestBuilder {
        <SendTestSmsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTestSmsRequestBuilder {
    phone_number: Option<String>,
}

impl SendTestSmsRequestBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendTestSmsRequest`].
    pub fn build(self) -> Result<SendTestSmsRequest, BuildError> {
        Ok(SendTestSmsRequest {
            phone_number: self.phone_number,
        })
    }
}
