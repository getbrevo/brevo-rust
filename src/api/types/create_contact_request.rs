pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateContactRequest {
    /// Pass the set of attributes and their values. The attribute's parameter should be passed in capital letter while creating a contact. Values that don't match the attribute type (e.g. text or string in a date attribute) will be ignored. **These attributes must be present in your Brevo account**. For eg: **{"FNAME":"Elly", "LNAME":"Roger", "COUNTRIES": ["India","China"]}**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, CreateContactRequestAttributesValue>>,
    /// Email address of the user. **Mandatory if "ext_id"  & "SMS" field is not passed.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Set this field to blacklist the contact for emails (emailBlacklisted = true)
    #[serde(rename = "emailBlacklisted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_blacklisted: Option<bool>,
    /// Pass your own Id to create a contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// Ids of the lists to add the contact to
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
    /// Set this field to blacklist the contact for SMS (smsBlacklisted = true)
    #[serde(rename = "smsBlacklisted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_blacklisted: Option<bool>,
    /// transactional email forbidden sender for contact. Use only for email Contact ( only available if updateEnabled = true )
    #[serde(rename = "smtpBlacklistSender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_blacklist_sender: Option<Vec<String>>,
    /// Facilitate to update the existing contact in the same request (updateEnabled = true)
    #[serde(rename = "updateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_enabled: Option<bool>,
    /// When true, if the contact being created shares an identifier (email, SMS, ext_id, whatsapp, landline) with an existing contact, the two contacts are force-merged. The contact with the most recent `last_modified` timestamp is retained; the other is deleted. When false (default), a 4xx error is returned on identifier conflict.
    #[serde(rename = "forceMerge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_merge: Option<bool>,
    /// When true, the response returns the `id` of the surviving contact after merge.
    #[serde(rename = "getId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_id: Option<bool>,
}

impl CreateContactRequest {
    pub fn builder() -> CreateContactRequestBuilder {
        <CreateContactRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateContactRequestBuilder {
    attributes: Option<HashMap<String, CreateContactRequestAttributesValue>>,
    email: Option<String>,
    email_blacklisted: Option<bool>,
    ext_id: Option<String>,
    list_ids: Option<Vec<i64>>,
    sms_blacklisted: Option<bool>,
    smtp_blacklist_sender: Option<Vec<String>>,
    update_enabled: Option<bool>,
    force_merge: Option<bool>,
    get_id: Option<bool>,
}

impl CreateContactRequestBuilder {
    pub fn attributes(
        mut self,
        value: HashMap<String, CreateContactRequestAttributesValue>,
    ) -> Self {
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

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
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

    pub fn update_enabled(mut self, value: bool) -> Self {
        self.update_enabled = Some(value);
        self
    }

    pub fn force_merge(mut self, value: bool) -> Self {
        self.force_merge = Some(value);
        self
    }

    pub fn get_id(mut self, value: bool) -> Self {
        self.get_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateContactRequest`].
    pub fn build(self) -> Result<CreateContactRequest, BuildError> {
        Ok(CreateContactRequest {
            attributes: self.attributes,
            email: self.email,
            email_blacklisted: self.email_blacklisted,
            ext_id: self.ext_id,
            list_ids: self.list_ids,
            sms_blacklisted: self.sms_blacklisted,
            smtp_blacklist_sender: self.smtp_blacklist_sender,
            update_enabled: self.update_enabled,
            force_merge: self.force_merge,
            get_id: self.get_id,
        })
    }
}
