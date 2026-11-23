pub use crate::prelude::*;

/// Status of the name associated with WhatsApp Phone number
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetWhatsAppConfigResponsePhoneNumberNameStatus {
    Approved,
    Pending,
    Rejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetWhatsAppConfigResponsePhoneNumberNameStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Approved => serializer.serialize_str("APPROVED"),
            Self::Pending => serializer.serialize_str("PENDING"),
            Self::Rejected => serializer.serialize_str("REJECTED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetWhatsAppConfigResponsePhoneNumberNameStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "APPROVED" => Ok(Self::Approved),
            "PENDING" => Ok(Self::Pending),
            "REJECTED" => Ok(Self::Rejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetWhatsAppConfigResponsePhoneNumberNameStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Approved => write!(f, "APPROVED"),
            Self::Pending => write!(f, "PENDING"),
            Self::Rejected => write!(f, "REJECTED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
