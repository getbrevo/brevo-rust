pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAbTestCampaignResultResponse {
    /// Click rate for current winning version
    #[serde(rename = "clickRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_rate: Option<String>,
    #[serde(rename = "clickedLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked_links: Option<GetAbTestCampaignResultResponseClickedLinks>,
    /// Open rate for current winning version
    #[serde(rename = "openRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<GetAbTestCampaignResultResponseStatistics>,
    /// Criteria chosen for winning version (Open/Click)
    #[serde(rename = "winningCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_criteria: Option<GetAbTestCampaignResultResponseWinningCriteria>,
    /// Subject Line of current winning version
    #[serde(rename = "winningSubjectLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_subject_line: Option<String>,
    /// Winning Campaign Info. pending = Campaign has been picked for sending and winning version is yet to be decided, tie = A tie happened between both the versions, notAvailable = Campaign has not yet been picked for sending.
    #[serde(rename = "winningVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_version: Option<GetAbTestCampaignResultResponseWinningVersion>,
    /// Open/Click rate for the winner version
    #[serde(rename = "winningVersionRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_version_rate: Option<String>,
}

impl GetAbTestCampaignResultResponse {
    pub fn builder() -> GetAbTestCampaignResultResponseBuilder {
        <GetAbTestCampaignResultResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAbTestCampaignResultResponseBuilder {
    click_rate: Option<String>,
    clicked_links: Option<GetAbTestCampaignResultResponseClickedLinks>,
    open_rate: Option<String>,
    statistics: Option<GetAbTestCampaignResultResponseStatistics>,
    winning_criteria: Option<GetAbTestCampaignResultResponseWinningCriteria>,
    winning_subject_line: Option<String>,
    winning_version: Option<GetAbTestCampaignResultResponseWinningVersion>,
    winning_version_rate: Option<String>,
}

impl GetAbTestCampaignResultResponseBuilder {
    pub fn click_rate(mut self, value: impl Into<String>) -> Self {
        self.click_rate = Some(value.into());
        self
    }

    pub fn clicked_links(mut self, value: GetAbTestCampaignResultResponseClickedLinks) -> Self {
        self.clicked_links = Some(value);
        self
    }

    pub fn open_rate(mut self, value: impl Into<String>) -> Self {
        self.open_rate = Some(value.into());
        self
    }

    pub fn statistics(mut self, value: GetAbTestCampaignResultResponseStatistics) -> Self {
        self.statistics = Some(value);
        self
    }

    pub fn winning_criteria(
        mut self,
        value: GetAbTestCampaignResultResponseWinningCriteria,
    ) -> Self {
        self.winning_criteria = Some(value);
        self
    }

    pub fn winning_subject_line(mut self, value: impl Into<String>) -> Self {
        self.winning_subject_line = Some(value.into());
        self
    }

    pub fn winning_version(mut self, value: GetAbTestCampaignResultResponseWinningVersion) -> Self {
        self.winning_version = Some(value);
        self
    }

    pub fn winning_version_rate(mut self, value: impl Into<String>) -> Self {
        self.winning_version_rate = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAbTestCampaignResultResponse`].
    pub fn build(self) -> Result<GetAbTestCampaignResultResponse, BuildError> {
        Ok(GetAbTestCampaignResultResponse {
            click_rate: self.click_rate,
            clicked_links: self.clicked_links,
            open_rate: self.open_rate,
            statistics: self.statistics,
            winning_criteria: self.winning_criteria,
            winning_subject_line: self.winning_subject_line,
            winning_version: self.winning_version,
            winning_version_rate: self.winning_version_rate,
        })
    }
}
