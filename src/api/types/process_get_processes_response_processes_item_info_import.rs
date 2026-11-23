pub use crate::prelude::*;

/// Import process details with URLs to CSV reports
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProcessesResponseProcessesItemInfoImport {
    /// URL to CSV file containing invalid email addresses, or null if none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_emails: Option<String>,
    /// URL to CSV file containing duplicate contact IDs, or null if none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_contact_id: Option<String>,
    /// URL to CSV file containing duplicate external IDs, or null if none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_ext_id: Option<String>,
    /// URL to CSV file containing duplicate email IDs, or null if none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_email_id: Option<String>,
    /// URL to CSV file containing duplicate phone numbers, or null if none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_phone_id: Option<String>,
    /// URL to CSV file containing duplicate WhatsApp numbers, or null if none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_whatsapp_id: Option<String>,
    /// URL to CSV file containing duplicate landline numbers, or null if none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_landline_number_id: Option<String>,
}

impl GetProcessesResponseProcessesItemInfoImport {
    pub fn builder() -> GetProcessesResponseProcessesItemInfoImportBuilder {
        <GetProcessesResponseProcessesItemInfoImportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProcessesResponseProcessesItemInfoImportBuilder {
    invalid_emails: Option<String>,
    duplicate_contact_id: Option<String>,
    duplicate_ext_id: Option<String>,
    duplicate_email_id: Option<String>,
    duplicate_phone_id: Option<String>,
    duplicate_whatsapp_id: Option<String>,
    duplicate_landline_number_id: Option<String>,
}

impl GetProcessesResponseProcessesItemInfoImportBuilder {
    pub fn invalid_emails(mut self, value: impl Into<String>) -> Self {
        self.invalid_emails = Some(value.into());
        self
    }

    pub fn duplicate_contact_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_contact_id = Some(value.into());
        self
    }

    pub fn duplicate_ext_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_ext_id = Some(value.into());
        self
    }

    pub fn duplicate_email_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_email_id = Some(value.into());
        self
    }

    pub fn duplicate_phone_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_phone_id = Some(value.into());
        self
    }

    pub fn duplicate_whatsapp_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_whatsapp_id = Some(value.into());
        self
    }

    pub fn duplicate_landline_number_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_landline_number_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetProcessesResponseProcessesItemInfoImport`].
    pub fn build(self) -> Result<GetProcessesResponseProcessesItemInfoImport, BuildError> {
        Ok(GetProcessesResponseProcessesItemInfoImport {
            invalid_emails: self.invalid_emails,
            duplicate_contact_id: self.duplicate_contact_id,
            duplicate_ext_id: self.duplicate_ext_id,
            duplicate_email_id: self.duplicate_email_id,
            duplicate_phone_id: self.duplicate_phone_id,
            duplicate_whatsapp_id: self.duplicate_whatsapp_id,
            duplicate_landline_number_id: self.duplicate_landline_number_id,
        })
    }
}
