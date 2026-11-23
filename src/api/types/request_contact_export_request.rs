pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestContactExportRequest {
    /// Set the filter for the contacts to be exported.
    #[serde(rename = "customContactFilter")]
    #[serde(default)]
    pub custom_contact_filter: RequestContactExportRequestCustomContactFilter,
    /// To avoid generating the email notification upon contact export, pass **true**
    #[serde(rename = "disableNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// List of all the attributes that you want to export. **These attributes must be present in your contact database. It is required if exportMandatoryAttributes is set false. ** For example: **['fname', 'lname', 'email']**
    #[serde(rename = "exportAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_attributes: Option<Vec<String>>,
    /// To export mandatory attributes like EMAIL, ADDED_TIME, MODIFIED_TIME
    #[serde(rename = "exportMandatoryAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_mandatory_attributes: Option<bool>,
    /// Export metadata of contacts such as _listIds, ADDED_TIME, MODIFIED_TIME.
    #[serde(rename = "exportMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_metadata: Option<Vec<String>>,
    /// Specifies whether the date fields createdAt, modifiedAt in the exported data should be returned in UTC format.
    #[serde(rename = "exportDateInUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_date_in_utc: Option<bool>,
    /// Export subscription status of contacts for email & sms marketting. Pass email_marketing to obtain the marketing email subscription status & sms_marketing to retrieve the marketing SMS status of the contact.
    #[serde(rename = "exportSubscriptionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_subscription_status: Option<Vec<String>>,
    /// Webhook that will be called once the export process is finished. For reference, https://help.brevo.com/hc/en-us/articles/360007666479
    #[serde(rename = "notifyUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
}

impl RequestContactExportRequest {
    pub fn builder() -> RequestContactExportRequestBuilder {
        <RequestContactExportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestContactExportRequestBuilder {
    custom_contact_filter: Option<RequestContactExportRequestCustomContactFilter>,
    disable_notification: Option<bool>,
    export_attributes: Option<Vec<String>>,
    export_mandatory_attributes: Option<bool>,
    export_metadata: Option<Vec<String>>,
    export_date_in_utc: Option<bool>,
    export_subscription_status: Option<Vec<String>>,
    notify_url: Option<String>,
}

impl RequestContactExportRequestBuilder {
    pub fn custom_contact_filter(
        mut self,
        value: RequestContactExportRequestCustomContactFilter,
    ) -> Self {
        self.custom_contact_filter = Some(value);
        self
    }

    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    pub fn export_attributes(mut self, value: Vec<String>) -> Self {
        self.export_attributes = Some(value);
        self
    }

    pub fn export_mandatory_attributes(mut self, value: bool) -> Self {
        self.export_mandatory_attributes = Some(value);
        self
    }

    pub fn export_metadata(mut self, value: Vec<String>) -> Self {
        self.export_metadata = Some(value);
        self
    }

    pub fn export_date_in_utc(mut self, value: bool) -> Self {
        self.export_date_in_utc = Some(value);
        self
    }

    pub fn export_subscription_status(mut self, value: Vec<String>) -> Self {
        self.export_subscription_status = Some(value);
        self
    }

    pub fn notify_url(mut self, value: impl Into<String>) -> Self {
        self.notify_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RequestContactExportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`custom_contact_filter`](RequestContactExportRequestBuilder::custom_contact_filter)
    pub fn build(self) -> Result<RequestContactExportRequest, BuildError> {
        Ok(RequestContactExportRequest {
            custom_contact_filter: self
                .custom_contact_filter
                .ok_or_else(|| BuildError::missing_field("custom_contact_filter"))?,
            disable_notification: self.disable_notification,
            export_attributes: self.export_attributes,
            export_mandatory_attributes: self.export_mandatory_attributes,
            export_metadata: self.export_metadata,
            export_date_in_utc: self.export_date_in_utc,
            export_subscription_status: self.export_subscription_status,
            notify_url: self.notify_url,
        })
    }
}
