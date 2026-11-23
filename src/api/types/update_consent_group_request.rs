pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateConsentGroupRequest {
    /// New name for the consent group (max 255 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New description (max 500 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New signup mode
    #[serde(rename = "signupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_mode: Option<UpdateConsentGroupRequestSignupMode>,
}

impl UpdateConsentGroupRequest {
    pub fn builder() -> UpdateConsentGroupRequestBuilder {
        <UpdateConsentGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateConsentGroupRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    signup_mode: Option<UpdateConsentGroupRequestSignupMode>,
}

impl UpdateConsentGroupRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn signup_mode(mut self, value: UpdateConsentGroupRequestSignupMode) -> Self {
        self.signup_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateConsentGroupRequest`].
    pub fn build(self) -> Result<UpdateConsentGroupRequest, BuildError> {
        Ok(UpdateConsentGroupRequest {
            name: self.name,
            description: self.description,
            signup_mode: self.signup_mode,
        })
    }
}
