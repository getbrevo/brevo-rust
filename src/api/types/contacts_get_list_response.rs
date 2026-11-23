pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetListResponse {
    /// ID of the list
    #[serde(default)]
    pub id: i64,
    /// Name of the list
    #[serde(default)]
    pub name: String,
    /// Number of blacklisted contacts in the list
    #[serde(rename = "totalBlacklisted")]
    #[serde(default)]
    pub total_blacklisted: i64,
    /// Number of contacts in the list
    #[serde(rename = "totalSubscribers")]
    #[serde(default)]
    pub total_subscribers: i64,
    /// Number of unique contacts in the list
    #[serde(rename = "uniqueSubscribers")]
    #[serde(default)]
    pub unique_subscribers: i64,
    #[serde(rename = "campaignStats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_stats: Option<Vec<GetListResponseCampaignStatsItem>>,
    /// Creation UTC date-time of the list (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Status telling if the list is dynamic or not (true=dynamic, false=not dynamic)
    #[serde(rename = "dynamicList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_list: Option<bool>,
    /// ID of the folder
    #[serde(rename = "folderId")]
    #[serde(default)]
    pub folder_id: i64,
}

impl GetListResponse {
    pub fn builder() -> GetListResponseBuilder {
        <GetListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetListResponseBuilder {
    id: Option<i64>,
    name: Option<String>,
    total_blacklisted: Option<i64>,
    total_subscribers: Option<i64>,
    unique_subscribers: Option<i64>,
    campaign_stats: Option<Vec<GetListResponseCampaignStatsItem>>,
    created_at: Option<String>,
    dynamic_list: Option<bool>,
    folder_id: Option<i64>,
}

impl GetListResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn total_blacklisted(mut self, value: i64) -> Self {
        self.total_blacklisted = Some(value);
        self
    }

    pub fn total_subscribers(mut self, value: i64) -> Self {
        self.total_subscribers = Some(value);
        self
    }

    pub fn unique_subscribers(mut self, value: i64) -> Self {
        self.unique_subscribers = Some(value);
        self
    }

    pub fn campaign_stats(mut self, value: Vec<GetListResponseCampaignStatsItem>) -> Self {
        self.campaign_stats = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn dynamic_list(mut self, value: bool) -> Self {
        self.dynamic_list = Some(value);
        self
    }

    pub fn folder_id(mut self, value: i64) -> Self {
        self.folder_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetListResponseBuilder::id)
    /// - [`name`](GetListResponseBuilder::name)
    /// - [`total_blacklisted`](GetListResponseBuilder::total_blacklisted)
    /// - [`total_subscribers`](GetListResponseBuilder::total_subscribers)
    /// - [`unique_subscribers`](GetListResponseBuilder::unique_subscribers)
    /// - [`created_at`](GetListResponseBuilder::created_at)
    /// - [`folder_id`](GetListResponseBuilder::folder_id)
    pub fn build(self) -> Result<GetListResponse, BuildError> {
        Ok(GetListResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            total_blacklisted: self
                .total_blacklisted
                .ok_or_else(|| BuildError::missing_field("total_blacklisted"))?,
            total_subscribers: self
                .total_subscribers
                .ok_or_else(|| BuildError::missing_field("total_subscribers"))?,
            unique_subscribers: self
                .unique_subscribers
                .ok_or_else(|| BuildError::missing_field("unique_subscribers"))?,
            campaign_stats: self.campaign_stats,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            dynamic_list: self.dynamic_list,
            folder_id: self
                .folder_id
                .ok_or_else(|| BuildError::missing_field("folder_id"))?,
        })
    }
}
