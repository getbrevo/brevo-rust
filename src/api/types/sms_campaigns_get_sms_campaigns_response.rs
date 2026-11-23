pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsCampaignsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<Vec<GetSmsCampaignsResponseCampaignsItem>>,
    /// Number of SMS campaigns retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl GetSmsCampaignsResponse {
    pub fn builder() -> GetSmsCampaignsResponseBuilder {
        <GetSmsCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsCampaignsResponseBuilder {
    campaigns: Option<Vec<GetSmsCampaignsResponseCampaignsItem>>,
    count: Option<i64>,
}

impl GetSmsCampaignsResponseBuilder {
    pub fn campaigns(mut self, value: Vec<GetSmsCampaignsResponseCampaignsItem>) -> Self {
        self.campaigns = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsCampaignsResponse`].
    pub fn build(self) -> Result<GetSmsCampaignsResponse, BuildError> {
        Ok(GetSmsCampaignsResponse {
            campaigns: self.campaigns,
            count: self.count,
        })
    }
}
