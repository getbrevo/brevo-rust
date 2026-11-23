pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetProcessResponse {
    /// Unique identifier of the process
    #[serde(default)]
    pub id: i64,
    /// Name/type of the process
    pub name: GetProcessResponseName,
    /// Current status of the process
    pub status: GetProcessResponseStatus,
    /// Additional process information, only returned for completed IMPORTUSER processes. Contains URLs to CSV files with details about problematic records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<GetProcessResponseInfo>,
    /// Download URL for completed export processes (returned for SEARCH_EXPORT_USERS, SEARCH_EXPORT_USERS_API, CAMPAIGN_USER_DETAILS, and EXPORT_WEBHOOK process types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_url: Option<String>,
}

impl GetProcessResponse {
    pub fn builder() -> GetProcessResponseBuilder {
        <GetProcessResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProcessResponseBuilder {
    id: Option<i64>,
    name: Option<GetProcessResponseName>,
    status: Option<GetProcessResponseStatus>,
    info: Option<GetProcessResponseInfo>,
    export_url: Option<String>,
}

impl GetProcessResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: GetProcessResponseName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn status(mut self, value: GetProcessResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn info(mut self, value: GetProcessResponseInfo) -> Self {
        self.info = Some(value);
        self
    }

    pub fn export_url(mut self, value: impl Into<String>) -> Self {
        self.export_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetProcessResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetProcessResponseBuilder::id)
    /// - [`name`](GetProcessResponseBuilder::name)
    /// - [`status`](GetProcessResponseBuilder::status)
    pub fn build(self) -> Result<GetProcessResponse, BuildError> {
        Ok(GetProcessResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            info: self.info,
            export_url: self.export_url,
        })
    }
}
