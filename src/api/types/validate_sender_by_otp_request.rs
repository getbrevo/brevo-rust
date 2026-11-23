pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidateSenderByOtpRequest {
    /// 6 digit OTP received on email
    #[serde(default)]
    pub otp: i64,
}

impl ValidateSenderByOtpRequest {
    pub fn builder() -> ValidateSenderByOtpRequestBuilder {
        <ValidateSenderByOtpRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateSenderByOtpRequestBuilder {
    otp: Option<i64>,
}

impl ValidateSenderByOtpRequestBuilder {
    pub fn otp(mut self, value: i64) -> Self {
        self.otp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ValidateSenderByOtpRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`otp`](ValidateSenderByOtpRequestBuilder::otp)
    pub fn build(self) -> Result<ValidateSenderByOtpRequest, BuildError> {
        Ok(ValidateSenderByOtpRequest {
            otp: self.otp.ok_or_else(|| BuildError::missing_field("otp"))?,
        })
    }
}
