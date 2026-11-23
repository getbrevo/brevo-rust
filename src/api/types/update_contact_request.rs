pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateContactRequest {
    /// Pass the set of attributes to be updated. **These attributes must be present in your account**. To update existing email address of a contact with the new one please pass EMAIL in attributes. For example, **{ "EMAIL":"newemail@domain.com", "FNAME":"Ellie", "LNAME":"Roger", "COUNTRIES":["India","China"]}**. The attribute's parameter should be passed in capital letter while updating a contact. Values that don't match the attribute type (e.g. text or string in a date attribute) will be ignored .Keep in mind transactional attributes can be updated the same way as normal attributes. Mobile Number in **SMS** field should be passed with proper country code. For example: **{"SMS":"+91xxxxxxxxxx"} or {"SMS":"0091xxxxxxxxxx"}**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, UpdateContactRequestAttributesValue>>,
    /// Set/unset this field to blacklist/allow the contact for emails (emailBlacklisted = true)
    #[serde(rename = "emailBlacklisted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_blacklisted: Option<bool>,
    /// Pass your own Id to update ext_id of a contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// Ids of the lists to add the contact to
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
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
    /// When true, if the contact being updated shares an identifier (email, SMS, ext_id, whatsapp, landline) with an existing contact, the two contacts are force-merged. The contact with the most recent `last_modified` timestamp is retained; the other is deleted. When false (default), a 4xx error is returned on identifier conflict.
    #[serde(rename = "forceMerge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_merge: Option<bool>,
    /// email_id for Email, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
    #[serde(rename = "identifierType")]
    #[serde(skip)]
    pub identifier_type: Option<UpdateContactRequestIdentifierType>,
}

impl UpdateContactRequest {
    pub fn builder() -> UpdateContactRequestBuilder {
        <UpdateContactRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateContactRequestBuilder {
    attributes: Option<HashMap<String, UpdateContactRequestAttributesValue>>,
    email_blacklisted: Option<bool>,
    ext_id: Option<String>,
    list_ids: Option<Vec<i64>>,
    sms_blacklisted: Option<bool>,
    smtp_blacklist_sender: Option<Vec<String>>,
    unlink_list_ids: Option<Vec<i64>>,
    force_merge: Option<bool>,
    identifier_type: Option<UpdateContactRequestIdentifierType>,
}

impl UpdateContactRequestBuilder {
    pub fn attributes(
        mut self,
        value: HashMap<String, UpdateContactRequestAttributesValue>,
    ) -> Self {
        self.attributes = Some(value);
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

    pub fn unlink_list_ids(mut self, value: Vec<i64>) -> Self {
        self.unlink_list_ids = Some(value);
        self
    }

    pub fn force_merge(mut self, value: bool) -> Self {
        self.force_merge = Some(value);
        self
    }

    pub fn identifier_type(mut self, value: UpdateContactRequestIdentifierType) -> Self {
        self.identifier_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateContactRequest`].
    pub fn build(self) -> Result<UpdateContactRequest, BuildError> {
        Ok(UpdateContactRequest {
            attributes: self.attributes,
            email_blacklisted: self.email_blacklisted,
            ext_id: self.ext_id,
            list_ids: self.list_ids,
            sms_blacklisted: self.sms_blacklisted,
            smtp_blacklist_sender: self.smtp_blacklist_sender,
            unlink_list_ids: self.unlink_list_ids,
            force_merge: self.force_merge,
            identifier_type: self.identifier_type,
        })
    }
}
