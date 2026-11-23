pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExportWebhooksHistoryRequest {
    /// Number of days in the past including today (positive
    /// integer). _Not compatible with 'startDate' and 'endDate'_
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Filter the history for a specific email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Mandatory if startDate is used. Ending date of the report
    /// (YYYY-MM-DD). Must be greater than equal to startDate
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Filter the history for a specific event type
    pub event: ExportWebhooksHistoryRequestEvent,
    /// Filter the history for a specific message id. Applicable
    /// only for transactional webhooks.
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Webhook URL to receive CSV file link
    #[serde(rename = "notifyURL")]
    #[serde(default)]
    pub notify_url: String,
    /// Sorting order of records (asc or desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Mandatory if endDate is used. Starting date of the history
    /// (YYYY-MM-DD). Must be lower than equal to endDate
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Filter the history based on webhook type
    pub r#type: ExportWebhooksHistoryRequestType,
    /// Filter the history for a specific webhook id
    #[serde(rename = "webhookId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<i64>,
}

impl ExportWebhooksHistoryRequest {
    pub fn builder() -> ExportWebhooksHistoryRequestBuilder {
        <ExportWebhooksHistoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportWebhooksHistoryRequestBuilder {
    days: Option<i64>,
    email: Option<String>,
    end_date: Option<String>,
    event: Option<ExportWebhooksHistoryRequestEvent>,
    message_id: Option<String>,
    notify_url: Option<String>,
    sort: Option<String>,
    start_date: Option<String>,
    r#type: Option<ExportWebhooksHistoryRequestType>,
    webhook_id: Option<i64>,
}

impl ExportWebhooksHistoryRequestBuilder {
    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn event(mut self, value: ExportWebhooksHistoryRequestEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn notify_url(mut self, value: impl Into<String>) -> Self {
        self.notify_url = Some(value.into());
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ExportWebhooksHistoryRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn webhook_id(mut self, value: i64) -> Self {
        self.webhook_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportWebhooksHistoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](ExportWebhooksHistoryRequestBuilder::event)
    /// - [`notify_url`](ExportWebhooksHistoryRequestBuilder::notify_url)
    /// - [`r#type`](ExportWebhooksHistoryRequestBuilder::r#type)
    pub fn build(self) -> Result<ExportWebhooksHistoryRequest, BuildError> {
        Ok(ExportWebhooksHistoryRequest {
            days: self.days,
            email: self.email,
            end_date: self.end_date,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            message_id: self.message_id,
            notify_url: self
                .notify_url
                .ok_or_else(|| BuildError::missing_field("notify_url"))?,
            sort: self.sort,
            start_date: self.start_date,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            webhook_id: self.webhook_id,
        })
    }
}
