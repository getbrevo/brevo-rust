pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactDetails {
    /// Set of attributes of the contact
    #[serde(default)]
    pub attributes: GetContactDetailsAttributes,
    /// Creation UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Email address of the contact for which you requested the details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Blacklist status for email campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "emailBlacklisted")]
    #[serde(default)]
    pub email_blacklisted: bool,
    /// ID of the contact for which you requested the details
    #[serde(default)]
    pub id: i64,
    #[serde(rename = "listIds")]
    #[serde(default)]
    pub list_ids: Vec<i64>,
    #[serde(rename = "listUnsubscribed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_unsubscribed: Option<Vec<i64>>,
    /// Last modification UTC date-time of the contact (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Blacklist status for SMS campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "smsBlacklisted")]
    #[serde(default)]
    pub sms_blacklisted: bool,
    /// Blacklist status for WhatsApp campaigns (true=blacklisted, false=not blacklisted)
    #[serde(rename = "whatsappBlacklisted")]
    #[serde(default)]
    pub whatsapp_blacklisted: bool,
    /// Consent groups the contact belongs to, with their subscription status. Only present when the Consent Groups feature is enabled for your account.
    #[serde(rename = "consentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_groups: Option<Vec<ConsentGroupItem>>,
}

impl GetContactDetails {
    pub fn builder() -> GetContactDetailsBuilder {
        <GetContactDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactDetailsBuilder {
    attributes: Option<GetContactDetailsAttributes>,
    created_at: Option<String>,
    email: Option<String>,
    email_blacklisted: Option<bool>,
    id: Option<i64>,
    list_ids: Option<Vec<i64>>,
    list_unsubscribed: Option<Vec<i64>>,
    modified_at: Option<String>,
    sms_blacklisted: Option<bool>,
    whatsapp_blacklisted: Option<bool>,
    consent_groups: Option<Vec<ConsentGroupItem>>,
}

impl GetContactDetailsBuilder {
    pub fn attributes(mut self, value: GetContactDetailsAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
        self
    }

    pub fn list_unsubscribed(mut self, value: Vec<i64>) -> Self {
        self.list_unsubscribed = Some(value);
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
        self
    }

    pub fn sms_blacklisted(mut self, value: bool) -> Self {
        self.sms_blacklisted = Some(value);
        self
    }

    pub fn whatsapp_blacklisted(mut self, value: bool) -> Self {
        self.whatsapp_blacklisted = Some(value);
        self
    }

    pub fn consent_groups(mut self, value: Vec<ConsentGroupItem>) -> Self {
        self.consent_groups = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](GetContactDetailsBuilder::attributes)
    /// - [`created_at`](GetContactDetailsBuilder::created_at)
    /// - [`email_blacklisted`](GetContactDetailsBuilder::email_blacklisted)
    /// - [`id`](GetContactDetailsBuilder::id)
    /// - [`list_ids`](GetContactDetailsBuilder::list_ids)
    /// - [`modified_at`](GetContactDetailsBuilder::modified_at)
    /// - [`sms_blacklisted`](GetContactDetailsBuilder::sms_blacklisted)
    /// - [`whatsapp_blacklisted`](GetContactDetailsBuilder::whatsapp_blacklisted)
    pub fn build(self) -> Result<GetContactDetails, BuildError> {
        Ok(GetContactDetails {
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            email: self.email,
            email_blacklisted: self
                .email_blacklisted
                .ok_or_else(|| BuildError::missing_field("email_blacklisted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            list_ids: self
                .list_ids
                .ok_or_else(|| BuildError::missing_field("list_ids"))?,
            list_unsubscribed: self.list_unsubscribed,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            sms_blacklisted: self
                .sms_blacklisted
                .ok_or_else(|| BuildError::missing_field("sms_blacklisted"))?,
            whatsapp_blacklisted: self
                .whatsapp_blacklisted
                .ok_or_else(|| BuildError::missing_field("whatsapp_blacklisted"))?,
            consent_groups: self.consent_groups,
        })
    }
}
