pub use crate::prelude::*;

/// Quality status of phone number associated with WhatsApp account. There are three quality ratings. example - **High (GREEN) , Medium (YELLOW) and Low(RED)**
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetWhatsAppConfigResponsePhoneNumberQuality {
    Green,
    Yellow,
    Red,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetWhatsAppConfigResponsePhoneNumberQuality {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Green => serializer.serialize_str("GREEN"),
            Self::Yellow => serializer.serialize_str("YELLOW"),
            Self::Red => serializer.serialize_str("RED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetWhatsAppConfigResponsePhoneNumberQuality {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "GREEN" => Ok(Self::Green),
            "YELLOW" => Ok(Self::Yellow),
            "RED" => Ok(Self::Red),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetWhatsAppConfigResponsePhoneNumberQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Green => write!(f, "GREEN"),
            Self::Yellow => write!(f, "YELLOW"),
            Self::Red => write!(f, "RED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
