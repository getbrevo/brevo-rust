pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateContactRequestIdentifierType {
    EmailId,
    ContactId,
    ExtId,
    PhoneId,
    WhatsappId,
    LandlineNumberId,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateContactRequestIdentifierType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EmailId => serializer.serialize_str("email_id"),
            Self::ContactId => serializer.serialize_str("contact_id"),
            Self::ExtId => serializer.serialize_str("ext_id"),
            Self::PhoneId => serializer.serialize_str("phone_id"),
            Self::WhatsappId => serializer.serialize_str("whatsapp_id"),
            Self::LandlineNumberId => serializer.serialize_str("landline_number_id"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateContactRequestIdentifierType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email_id" => Ok(Self::EmailId),
            "contact_id" => Ok(Self::ContactId),
            "ext_id" => Ok(Self::ExtId),
            "phone_id" => Ok(Self::PhoneId),
            "whatsapp_id" => Ok(Self::WhatsappId),
            "landline_number_id" => Ok(Self::LandlineNumberId),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateContactRequestIdentifierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmailId => write!(f, "email_id"),
            Self::ContactId => write!(f, "contact_id"),
            Self::ExtId => write!(f, "ext_id"),
            Self::PhoneId => write!(f, "phone_id"),
            Self::WhatsappId => write!(f, "whatsapp_id"),
            Self::LandlineNumberId => write!(f, "landline_number_id"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
