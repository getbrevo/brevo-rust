pub use crate::prelude::*;

/// Query parameters for getEmailCampaign
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailCampaignQueryRequest {
    /// Filter on the type of statistics required. Example: **globalStats** value will only fetch globalStats info of the campaign in the returned response. `statsByDevice` and `statsByBrowser` are only available when retrieving a single campaign (not in the list endpoint).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<GetEmailCampaignRequestStatistics>,
    /// Use this flag to exclude htmlContent from the response body. If set to **true**, htmlContent field will be returned as empty string in the response body
    #[serde(rename = "excludeHtmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_html_content: Option<bool>,
}

impl GetEmailCampaignQueryRequest {
    pub fn builder() -> GetEmailCampaignQueryRequestBuilder {
        <GetEmailCampaignQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignQueryRequestBuilder {
    statistics: Option<GetEmailCampaignRequestStatistics>,
    exclude_html_content: Option<bool>,
}

impl GetEmailCampaignQueryRequestBuilder {
    pub fn statistics(mut self, value: GetEmailCampaignRequestStatistics) -> Self {
        self.statistics = Some(value);
        self
    }

    pub fn exclude_html_content(mut self, value: bool) -> Self {
        self.exclude_html_content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailCampaignQueryRequest`].
    pub fn build(self) -> Result<GetEmailCampaignQueryRequest, BuildError> {
        Ok(GetEmailCampaignQueryRequest {
            statistics: self.statistics,
            exclude_html_content: self.exclude_html_content,
        })
    }
}
