pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportContactsRequest {
    /// To disable email notification
    #[serde(rename = "disableNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// To blacklist all the contacts for email
    #[serde(rename = "emailBlacklist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_blacklist: Option<bool>,
    /// To facilitate the choice to erase any attribute of the existing contacts with empty value. emptyContactsAttributes = true means the empty fields in your import will erase any attribute that currently contain data in Brevo, & emptyContactsAttributes = false means the empty fields will not affect your existing data ( **only available if `updateExistingContacts` set to true **)
    #[serde(rename = "emptyContactsAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_contacts_attributes: Option<bool>,
    /// **Mandatory if fileUrl and jsonBody is not defined.** CSV content to be imported. Use semicolon to separate multiple attributes. **Maximum allowed file body size is 10MB** . However we recommend a safe limit of around 8 MB to avoid the issues caused due to increase of file body size while parsing. Please use fileUrl instead to import bigger files.
    #[serde(rename = "fileBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_body: Option<String>,
    /// **Mandatory if fileBody and jsonBody is not defined.** URL of the file to be imported (**no local file**). Possible file formats: #### .txt, .csv, .json
    #[serde(rename = "fileUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// **Mandatory if fileUrl and fileBody is not defined.** JSON content to be imported. **Maximum allowed json body size is 10MB** . However we recommend a safe limit of around 8 MB to avoid the issues caused due to increase of json body size while parsing. Please use fileUrl instead to import bigger files.
    #[serde(rename = "jsonBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<Vec<ImportContactsRequestJsonBodyItem>>,
    /// **Mandatory if newList is not defined.** Ids of the lists in which the contacts shall be imported. For example, **[2, 4, 7]**.
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
    /// **Optional.** Ids of the consent groups to which all imported contacts will be added. Requires consent groups to be enabled for the organisation. For example, **[1, 3]**.
    #[serde(rename = "consentGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_group_ids: Option<Vec<i64>>,
    /// To create a new list and import the contacts into it, pass the listName and an optional folderId.
    #[serde(rename = "newList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_list: Option<ImportContactsRequestNewList>,
    /// URL that will be called once the import process is finished. For reference, https://help.brevo.com/hc/en-us/articles/360007666479
    #[serde(rename = "notifyUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    /// To blacklist all the contacts for sms
    #[serde(rename = "smsBlacklist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_blacklist: Option<bool>,
    /// To facilitate the choice to update the existing contacts
    #[serde(rename = "updateExistingContacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_existing_contacts: Option<bool>,
}

impl ImportContactsRequest {
    pub fn builder() -> ImportContactsRequestBuilder {
        <ImportContactsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportContactsRequestBuilder {
    disable_notification: Option<bool>,
    email_blacklist: Option<bool>,
    empty_contacts_attributes: Option<bool>,
    file_body: Option<String>,
    file_url: Option<String>,
    json_body: Option<Vec<ImportContactsRequestJsonBodyItem>>,
    list_ids: Option<Vec<i64>>,
    consent_group_ids: Option<Vec<i64>>,
    new_list: Option<ImportContactsRequestNewList>,
    notify_url: Option<String>,
    sms_blacklist: Option<bool>,
    update_existing_contacts: Option<bool>,
}

impl ImportContactsRequestBuilder {
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    pub fn email_blacklist(mut self, value: bool) -> Self {
        self.email_blacklist = Some(value);
        self
    }

    pub fn empty_contacts_attributes(mut self, value: bool) -> Self {
        self.empty_contacts_attributes = Some(value);
        self
    }

    pub fn file_body(mut self, value: impl Into<String>) -> Self {
        self.file_body = Some(value.into());
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    pub fn json_body(mut self, value: Vec<ImportContactsRequestJsonBodyItem>) -> Self {
        self.json_body = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
        self
    }

    pub fn consent_group_ids(mut self, value: Vec<i64>) -> Self {
        self.consent_group_ids = Some(value);
        self
    }

    pub fn new_list(mut self, value: ImportContactsRequestNewList) -> Self {
        self.new_list = Some(value);
        self
    }

    pub fn notify_url(mut self, value: impl Into<String>) -> Self {
        self.notify_url = Some(value.into());
        self
    }

    pub fn sms_blacklist(mut self, value: bool) -> Self {
        self.sms_blacklist = Some(value);
        self
    }

    pub fn update_existing_contacts(mut self, value: bool) -> Self {
        self.update_existing_contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportContactsRequest`].
    pub fn build(self) -> Result<ImportContactsRequest, BuildError> {
        Ok(ImportContactsRequest {
            disable_notification: self.disable_notification,
            email_blacklist: self.email_blacklist,
            empty_contacts_attributes: self.empty_contacts_attributes,
            file_body: self.file_body,
            file_url: self.file_url,
            json_body: self.json_body,
            list_ids: self.list_ids,
            consent_group_ids: self.consent_group_ids,
            new_list: self.new_list,
            notify_url: self.notify_url,
            sms_blacklist: self.sms_blacklist,
            update_existing_contacts: self.update_existing_contacts,
        })
    }
}
