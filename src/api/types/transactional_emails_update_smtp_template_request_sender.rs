pub use crate::prelude::*;

/// Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSmtpTemplateRequestSender {
    /// Email of the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Select the sender for the template on the basis of sender id. _In order to select a sender with specific pool of IP’s, dedicated ip users shall pass id (instead of email)_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Name of the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateSmtpTemplateRequestSender {
    pub fn builder() -> UpdateSmtpTemplateRequestSenderBuilder {
        <UpdateSmtpTemplateRequestSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSmtpTemplateRequestSenderBuilder {
    email: Option<String>,
    id: Option<i64>,
    name: Option<String>,
}

impl UpdateSmtpTemplateRequestSenderBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSmtpTemplateRequestSender`].
    pub fn build(self) -> Result<UpdateSmtpTemplateRequestSender, BuildError> {
        Ok(UpdateSmtpTemplateRequestSender {
            email: self.email,
            id: self.id,
            name: self.name,
        })
    }
}
