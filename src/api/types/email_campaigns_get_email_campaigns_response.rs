pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEmailCampaignsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<Vec<GetEmailCampaignsResponseCampaignsItem>>,
    /// Number of Email campaigns retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl GetEmailCampaignsResponse {
    pub fn builder() -> GetEmailCampaignsResponseBuilder {
        <GetEmailCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignsResponseBuilder {
    campaigns: Option<Vec<GetEmailCampaignsResponseCampaignsItem>>,
    count: Option<i64>,
}

impl GetEmailCampaignsResponseBuilder {
    pub fn campaigns(mut self, value: Vec<GetEmailCampaignsResponseCampaignsItem>) -> Self {
        self.campaigns = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailCampaignsResponse`].
    pub fn build(self) -> Result<GetEmailCampaignsResponse, BuildError> {
        Ok(GetEmailCampaignsResponse {
            campaigns: self.campaigns,
            count: self.count,
        })
    }
}
