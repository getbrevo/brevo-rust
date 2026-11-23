pub use crate::prelude::*;

/// Identifies the contact associated with the event. At least one identifier is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEventRequestIdentifiers {
    /// Internal unique contact ID. When present, this takes priority over all other identifiers for event attribution and contact resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Email Id associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// ext_id associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// landline_number associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landline_number_id: Option<String>,
    /// SMS associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_id: Option<String>,
    /// whatsapp associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_id: Option<String>,
}

impl CreateEventRequestIdentifiers {
    pub fn builder() -> CreateEventRequestIdentifiersBuilder {
        <CreateEventRequestIdentifiersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEventRequestIdentifiersBuilder {
    contact_id: Option<i64>,
    email_id: Option<String>,
    ext_id: Option<String>,
    landline_number_id: Option<String>,
    phone_id: Option<String>,
    whatsapp_id: Option<String>,
}

impl CreateEventRequestIdentifiersBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn landline_number_id(mut self, value: impl Into<String>) -> Self {
        self.landline_number_id = Some(value.into());
        self
    }

    pub fn phone_id(mut self, value: impl Into<String>) -> Self {
        self.phone_id = Some(value.into());
        self
    }

    pub fn whatsapp_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEventRequestIdentifiers`].
    pub fn build(self) -> Result<CreateEventRequestIdentifiers, BuildError> {
        Ok(CreateEventRequestIdentifiers {
            contact_id: self.contact_id,
            email_id: self.email_id,
            ext_id: self.ext_id,
            landline_number_id: self.landline_number_id,
            phone_id: self.phone_id,
            whatsapp_id: self.whatsapp_id,
        })
    }
}
