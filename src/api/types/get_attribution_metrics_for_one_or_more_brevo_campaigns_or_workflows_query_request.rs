pub use crate::prelude::*;

/// Query parameters for getAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflows
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest {
    /// When getting metrics for a specific period, define the starting datetime in RFC3339 format
    #[serde(rename = "periodFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub period_from: Option<DateTime<FixedOffset>>,
    /// When getting metrics for a specific period, define the end datetime in RFC3339 format
    #[serde(rename = "periodTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub period_to: Option<DateTime<FixedOffset>>,
    /// The email campaign ID(s) to get metrics for
    #[serde(rename = "emailCampaignId[]")]
    #[serde(default)]
    pub email_campaign_id_array: Vec<Option<String>>,
    /// The SMS campaign ID(s) to get metrics for
    #[serde(rename = "smsCampaignId[]")]
    #[serde(default)]
    pub sms_campaign_id_array: Vec<Option<String>>,
    /// The automation workflow ID(s) to get email attribution metrics for
    #[serde(rename = "automationWorkflowEmailId[]")]
    #[serde(default)]
    pub automation_workflow_email_id_array: Vec<Option<String>>,
    /// The automation workflow ID(s) to get SMS attribution metrics for
    #[serde(rename = "automationWorkflowSmsId[]")]
    #[serde(default)]
    pub automation_workflow_sms_id_array: Vec<Option<String>>,
}

impl GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest {
    pub fn builder() -> GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder
    {
        <GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder {
    period_from: Option<DateTime<FixedOffset>>,
    period_to: Option<DateTime<FixedOffset>>,
    email_campaign_id_array: Option<Vec<Option<String>>>,
    sms_campaign_id_array: Option<Vec<Option<String>>>,
    automation_workflow_email_id_array: Option<Vec<Option<String>>>,
    automation_workflow_sms_id_array: Option<Vec<Option<String>>>,
}

impl GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder {
    pub fn period_from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.period_from = Some(value);
        self
    }

    pub fn period_to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.period_to = Some(value);
        self
    }

    pub fn email_campaign_id_array(mut self, value: Vec<Option<String>>) -> Self {
        self.email_campaign_id_array = Some(value);
        self
    }

    pub fn sms_campaign_id_array(mut self, value: Vec<Option<String>>) -> Self {
        self.sms_campaign_id_array = Some(value);
        self
    }

    pub fn automation_workflow_email_id_array(mut self, value: Vec<Option<String>>) -> Self {
        self.automation_workflow_email_id_array = Some(value);
        self
    }

    pub fn automation_workflow_sms_id_array(mut self, value: Vec<Option<String>>) -> Self {
        self.automation_workflow_sms_id_array = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_campaign_id_array`](GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder::email_campaign_id_array)
    /// - [`sms_campaign_id_array`](GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder::sms_campaign_id_array)
    /// - [`automation_workflow_email_id_array`](GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder::automation_workflow_email_id_array)
    /// - [`automation_workflow_sms_id_array`](GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequestBuilder::automation_workflow_sms_id_array)
    pub fn build(
        self,
    ) -> Result<GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest, BuildError>
    {
        Ok(
            GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest {
                period_from: self.period_from,
                period_to: self.period_to,
                email_campaign_id_array: self
                    .email_campaign_id_array
                    .ok_or_else(|| BuildError::missing_field("email_campaign_id_array"))?,
                sms_campaign_id_array: self
                    .sms_campaign_id_array
                    .ok_or_else(|| BuildError::missing_field("sms_campaign_id_array"))?,
                automation_workflow_email_id_array: self
                    .automation_workflow_email_id_array
                    .ok_or_else(|| {
                        BuildError::missing_field("automation_workflow_email_id_array")
                    })?,
                automation_workflow_sms_id_array: self
                    .automation_workflow_sms_id_array
                    .ok_or_else(|| BuildError::missing_field("automation_workflow_sms_id_array"))?,
            },
        )
    }
}
