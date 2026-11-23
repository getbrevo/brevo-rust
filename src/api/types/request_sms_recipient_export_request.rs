pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestSmsRecipientExportRequest {
    /// URL that will be called once the export process is finished. For reference, https://help.brevo.com/hc/en-us/articles/360007666479
    #[serde(rename = "notifyURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// Filter the recipients based on how they interacted with the campaign
    #[serde(rename = "recipientsType")]
    pub recipients_type: RequestSmsRecipientExportRequestRecipientsType,
}

impl RequestSmsRecipientExportRequest {
    pub fn builder() -> RequestSmsRecipientExportRequestBuilder {
        <RequestSmsRecipientExportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestSmsRecipientExportRequestBuilder {
    notify_url: Option<String>,
    recipients_type: Option<RequestSmsRecipientExportRequestRecipientsType>,
}

impl RequestSmsRecipientExportRequestBuilder {
    pub fn notify_url(mut self, value: impl Into<String>) -> Self {
        self.notify_url = Some(value.into());
        self
    }

    pub fn recipients_type(
        mut self,
        value: RequestSmsRecipientExportRequestRecipientsType,
    ) -> Self {
        self.recipients_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestSmsRecipientExportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recipients_type`](RequestSmsRecipientExportRequestBuilder::recipients_type)
    pub fn build(self) -> Result<RequestSmsRecipientExportRequest, BuildError> {
        Ok(RequestSmsRecipientExportRequest {
            notify_url: self.notify_url,
            recipients_type: self
                .recipients_type
                .ok_or_else(|| BuildError::missing_field("recipients_type"))?,
        })
    }
}
