pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSenderRequest {
    /// From email to use for the sender. A verification email will be
    /// sent to this address.
    #[serde(default)]
    pub email: String,
    /// **Mandatory in case of dedicated IP**. IPs to associate to the
    /// sender. Not required for standard accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<CreateSenderRequestIpsItem>>,
    /// From Name to use for the sender
    #[serde(default)]
    pub name: String,
}

impl CreateSenderRequest {
    pub fn builder() -> CreateSenderRequestBuilder {
        <CreateSenderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSenderRequestBuilder {
    email: Option<String>,
    ips: Option<Vec<CreateSenderRequestIpsItem>>,
    name: Option<String>,
}

impl CreateSenderRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn ips(mut self, value: Vec<CreateSenderRequestIpsItem>) -> Self {
        self.ips = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSenderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](CreateSenderRequestBuilder::email)
    /// - [`name`](CreateSenderRequestBuilder::name)
    pub fn build(self) -> Result<CreateSenderRequest, BuildError> {
        Ok(CreateSenderRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            ips: self.ips,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
