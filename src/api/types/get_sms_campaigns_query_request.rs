pub use crate::prelude::*;

/// Query parameters for getSmsCampaigns
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsCampaignsQueryRequest {
    /// Status of campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetSmsCampaignsRequestStatus>,
    /// **Mandatory if endDate is used.** Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent SMS campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. `startDate` must not be in the future.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent SMS campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. `endDate` must not be in the future.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Beginning point in the list to retrieve from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetSmsCampaignsRequestSort>,
}

impl GetSmsCampaignsQueryRequest {
    pub fn builder() -> GetSmsCampaignsQueryRequestBuilder {
        <GetSmsCampaignsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsCampaignsQueryRequestBuilder {
    status: Option<GetSmsCampaignsRequestStatus>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetSmsCampaignsRequestSort>,
}

impl GetSmsCampaignsQueryRequestBuilder {
    pub fn status(mut self, value: GetSmsCampaignsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetSmsCampaignsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsCampaignsQueryRequest`].
    pub fn build(self) -> Result<GetSmsCampaignsQueryRequest, BuildError> {
        Ok(GetSmsCampaignsQueryRequest {
            status: self.status,
            start_date: self.start_date,
            end_date: self.end_date,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
        })
    }
}
