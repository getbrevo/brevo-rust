pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSenderRequest {
    /// From Email to update the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// **Only in case of dedicated IP**. IPs to associate to the
    /// sender. If passed, will replace all the existing IPs. Not required for standard accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<UpdateSenderRequestIpsItem>>,
    /// From Name to update the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateSenderRequest {
    pub fn builder() -> UpdateSenderRequestBuilder {
        <UpdateSenderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSenderRequestBuilder {
    email: Option<String>,
    ips: Option<Vec<UpdateSenderRequestIpsItem>>,
    name: Option<String>,
}

impl UpdateSenderRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn ips(mut self, value: Vec<UpdateSenderRequestIpsItem>) -> Self {
        self.ips = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSenderRequest`].
    pub fn build(self) -> Result<UpdateSenderRequest, BuildError> {
        Ok(UpdateSenderRequest {
            email: self.email,
            ips: self.ips,
            name: self.name,
        })
    }
}
