pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidateRewardResponse {
    /// Boolean value to authorize a reward or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize: Option<bool>,
}

impl ValidateRewardResponse {
    pub fn builder() -> ValidateRewardResponseBuilder {
        <ValidateRewardResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateRewardResponseBuilder {
    authorize: Option<bool>,
}

impl ValidateRewardResponseBuilder {
    pub fn authorize(mut self, value: bool) -> Self {
        self.authorize = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ValidateRewardResponse`].
    pub fn build(self) -> Result<ValidateRewardResponse, BuildError> {
        Ok(ValidateRewardResponse {
            authorize: self.authorize,
        })
    }
}
