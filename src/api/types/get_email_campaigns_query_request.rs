pub use crate::prelude::*;

/// Query parameters for getEmailCampaigns
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailCampaignsQueryRequest {
    /// Filter on the type of the campaigns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GetEmailCampaignsRequestType>,
    /// Filter on the status of the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetEmailCampaignsRequestStatus>,
    /// Filter on the type of statistics required. Example: **globalStats** value will only fetch globalStats info of the campaign in the returned response. This option only returns data for events that occurred in the last 6 months. For older campaigns, it is advisable to use the **Get Campaign Report** endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<GetEmailCampaignsRequestStatistics>,
    /// **Mandatory if endDate is used.** Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. The date range between `startDate` and `endDate` must not exceed 2 years. `startDate` must not be in the future.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. The date range between `startDate` and `endDate` must not exceed 2 years. `endDate` must not be in the future.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetEmailCampaignsRequestSort>,
    /// Use this flag to exclude htmlContent from the response body. If set to **true**, the htmlContent field will be returned as an empty string in the response body.
    #[serde(rename = "excludeHtmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_html_content: Option<bool>,
    /// Use this flag to filter out campaigns that have a PDF attachment. If set to **true**, only campaigns without a PDF attachment (or with no attachment at all) will be returned.
    #[serde(rename = "excludePdfAttachment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_pdf_attachment: Option<bool>,
}

impl GetEmailCampaignsQueryRequest {
    pub fn builder() -> GetEmailCampaignsQueryRequestBuilder {
        <GetEmailCampaignsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignsQueryRequestBuilder {
    r#type: Option<GetEmailCampaignsRequestType>,
    status: Option<GetEmailCampaignsRequestStatus>,
    statistics: Option<GetEmailCampaignsRequestStatistics>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetEmailCampaignsRequestSort>,
    exclude_html_content: Option<bool>,
    exclude_pdf_attachment: Option<bool>,
}

impl GetEmailCampaignsQueryRequestBuilder {
    pub fn r#type(mut self, value: GetEmailCampaignsRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: GetEmailCampaignsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn statistics(mut self, value: GetEmailCampaignsRequestStatistics) -> Self {
        self.statistics = Some(value);
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

    pub fn sort(mut self, value: GetEmailCampaignsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn exclude_html_content(mut self, value: bool) -> Self {
        self.exclude_html_content = Some(value);
        self
    }

    pub fn exclude_pdf_attachment(mut self, value: bool) -> Self {
        self.exclude_pdf_attachment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailCampaignsQueryRequest`].
    pub fn build(self) -> Result<GetEmailCampaignsQueryRequest, BuildError> {
        Ok(GetEmailCampaignsQueryRequest {
            r#type: self.r#type,
            status: self.status,
            statistics: self.statistics,
            start_date: self.start_date,
            end_date: self.end_date,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            exclude_html_content: self.exclude_html_content,
            exclude_pdf_attachment: self.exclude_pdf_attachment,
        })
    }
}
