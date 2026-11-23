pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailCampaignResponseSender {
    /// Sender email of the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Sender id of the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Sender name of the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetEmailCampaignResponseSender {
    pub fn builder() -> GetEmailCampaignResponseSenderBuilder {
        <GetEmailCampaignResponseSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignResponseSenderBuilder {
    email: Option<String>,
    id: Option<i64>,
    name: Option<String>,
}

impl GetEmailCampaignResponseSenderBuilder {
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

    /// Consumes the builder and constructs a [`GetEmailCampaignResponseSender`].
    pub fn build(self) -> Result<GetEmailCampaignResponseSender, BuildError> {
        Ok(GetEmailCampaignResponseSender {
            email: self.email,
            id: self.id,
            name: self.name,
        })
    }
}
