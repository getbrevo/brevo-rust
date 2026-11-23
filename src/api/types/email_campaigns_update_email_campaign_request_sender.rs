pub use crate::prelude::*;

/// Sender details including id or email and name (optional). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEmailCampaignRequestSender {
    /// Sender email from which the campaign emails are sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Select the sender for the campaign on the basis of sender id. **In order to select a sender with specific pool of IP’s, dedicated ip users shall pass id (instead of email)**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Sender Name from which the campaign emails are sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateEmailCampaignRequestSender {
    pub fn builder() -> UpdateEmailCampaignRequestSenderBuilder {
        <UpdateEmailCampaignRequestSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailCampaignRequestSenderBuilder {
    email: Option<String>,
    id: Option<i64>,
    name: Option<String>,
}

impl UpdateEmailCampaignRequestSenderBuilder {
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

    /// Consumes the builder and constructs a [`UpdateEmailCampaignRequestSender`].
    pub fn build(self) -> Result<UpdateEmailCampaignRequestSender, BuildError> {
        Ok(UpdateEmailCampaignRequestSender {
            email: self.email,
            id: self.id,
            name: self.name,
        })
    }
}
