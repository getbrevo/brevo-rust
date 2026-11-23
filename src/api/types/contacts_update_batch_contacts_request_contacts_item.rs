pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateBatchContactsRequestContactsItem {
    /// Pass the set of attributes to be updated. **These attributes must be present in your account**. To update existing email address of a contact with the new one please pass EMAIL in attribtes. For example, **{ "EMAIL":"newemail@domain.com", "FNAME":"Ellie", "LNAME":"Roger"}**. Keep in mind transactional attributes can be updated the same way as normal attributes. Mobile Number in **SMS** field should be passed with proper country code. For example: **{"SMS":"+91xxxxxxxxxx"} or {"SMS":"0091xxxxxxxxxx"}**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Email address of the user to be updated (For each operation only pass one of the supported contact identifiers. Email, id or sms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Set/unset this field to blacklist/allow the contact for emails (emailBlacklisted = true)
    #[serde(rename = "emailBlacklisted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_blacklisted: Option<bool>,
    /// Pass your own Id to update ext_id of a contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// id of the user to be updated (For each operation only pass one of the supported contact identifiers. Email, id or sms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Ids of the lists to add the contact to
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
    /// SMS of the user to be updated (For each operation only pass one of the supported contact identifiers. Email, id or sms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<String>,
    /// Set/unset this field to blacklist/allow the contact for SMS (smsBlacklisted = true)
    #[serde(rename = "smsBlacklisted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_blacklisted: Option<bool>,
    /// transactional email forbidden sender for contact. Use only for email Contact
    #[serde(rename = "smtpBlacklistSender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_blacklist_sender: Option<Vec<String>>,
    /// Ids of the lists to remove the contact from
    #[serde(rename = "unlinkListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlink_list_ids: Option<Vec<i64>>,
}

impl UpdateBatchContactsRequestContactsItem {
    pub fn builder() -> UpdateBatchContactsRequestContactsItemBuilder {
        <UpdateBatchContactsRequestContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBatchContactsRequestContactsItemBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    email: Option<String>,
    email_blacklisted: Option<bool>,
    ext_id: Option<String>,
    id: Option<i64>,
    list_ids: Option<Vec<i64>>,
    sms: Option<String>,
    sms_blacklisted: Option<bool>,
    smtp_blacklist_sender: Option<Vec<String>>,
    unlink_list_ids: Option<Vec<i64>>,
}

impl UpdateBatchContactsRequestContactsItemBuilder {
    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn email_blacklisted(mut self, value: bool) -> Self {
        self.email_blacklisted = Some(value);
        self
    }

    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
        self
    }

    pub fn sms(mut self, value: impl Into<String>) -> Self {
        self.sms = Some(value.into());
        self
    }

    pub fn sms_blacklisted(mut self, value: bool) -> Self {
        self.sms_blacklisted = Some(value);
        self
    }

    pub fn smtp_blacklist_sender(mut self, value: Vec<String>) -> Self {
        self.smtp_blacklist_sender = Some(value);
        self
    }

    pub fn unlink_list_ids(mut self, value: Vec<i64>) -> Self {
        self.unlink_list_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBatchContactsRequestContactsItem`].
    pub fn build(self) -> Result<UpdateBatchContactsRequestContactsItem, BuildError> {
        Ok(UpdateBatchContactsRequestContactsItem {
            attributes: self.attributes,
            email: self.email,
            email_blacklisted: self.email_blacklisted,
            ext_id: self.ext_id,
            id: self.id,
            list_ids: self.list_ids,
            sms: self.sms,
            sms_blacklisted: self.sms_blacklisted,
            smtp_blacklist_sender: self.smtp_blacklist_sender,
            unlink_list_ids: self.unlink_list_ids,
        })
    }
}
