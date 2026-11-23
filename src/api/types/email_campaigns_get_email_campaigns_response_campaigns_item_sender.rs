pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailCampaignsResponseCampaignsItemSender {
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

impl GetEmailCampaignsResponseCampaignsItemSender {
    pub fn builder() -> GetEmailCampaignsResponseCampaignsItemSenderBuilder {
        <GetEmailCampaignsResponseCampaignsItemSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignsResponseCampaignsItemSenderBuilder {
    email: Option<String>,
    id: Option<i64>,
    name: Option<String>,
}

impl GetEmailCampaignsResponseCampaignsItemSenderBuilder {
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

    /// Consumes the builder and constructs a [`GetEmailCampaignsResponseCampaignsItemSender`].
    pub fn build(self) -> Result<GetEmailCampaignsResponseCampaignsItemSender, BuildError> {
        Ok(GetEmailCampaignsResponseCampaignsItemSender {
            email: self.email,
            id: self.id,
            name: self.name,
        })
    }
}
