pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthenticateDomainResponse {
    /// Domain name that was authenticated
    #[serde(default)]
    pub domain_name: String,
    /// Success message confirming authentication
    #[serde(default)]
    pub message: String,
}

impl AuthenticateDomainResponse {
    pub fn builder() -> AuthenticateDomainResponseBuilder {
        <AuthenticateDomainResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthenticateDomainResponseBuilder {
    domain_name: Option<String>,
    message: Option<String>,
}

impl AuthenticateDomainResponseBuilder {
    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthenticateDomainResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain_name`](AuthenticateDomainResponseBuilder::domain_name)
    /// - [`message`](AuthenticateDomainResponseBuilder::message)
    pub fn build(self) -> Result<AuthenticateDomainResponse, BuildError> {
        Ok(AuthenticateDomainResponse {
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
