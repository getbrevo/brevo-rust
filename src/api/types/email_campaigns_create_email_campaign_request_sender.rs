pub use crate::prelude::*;

/// Sender details including id or email and name (optional). Only one of either Sender’s email or Sender’s ID shall be passed in one request at a time. Passing both `email` and `id` will result in an error. For example: **{"name":"xyz", "email":"example@abc.com"}** or **{"name":"xyz", "id":123}**
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEmailCampaignRequestSender {
    /// Sender email. Must be a valid email address. Cannot be used together with `id` in the same request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Select the sender for the campaign on the basis of sender id. Cannot be used together with `email` in the same request. _In order to select a sender with a specific pool of IPs, dedicated IP users shall pass id (instead of email)._
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Sender Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateEmailCampaignRequestSender {
    pub fn builder() -> CreateEmailCampaignRequestSenderBuilder {
        <CreateEmailCampaignRequestSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEmailCampaignRequestSenderBuilder {
    email: Option<String>,
    id: Option<i64>,
    name: Option<String>,
}

impl CreateEmailCampaignRequestSenderBuilder {
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

    /// Consumes the builder and constructs a [`CreateEmailCampaignRequestSender`].
    pub fn build(self) -> Result<CreateEmailCampaignRequestSender, BuildError> {
        Ok(CreateEmailCampaignRequestSender {
            email: self.email,
            id: self.id,
            name: self.name,
        })
    }
}
