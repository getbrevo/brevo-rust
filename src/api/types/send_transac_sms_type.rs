pub use crate::prelude::*;

/// Type of the SMS. Marketing SMS messages are those sent typically with marketing content. Transactional SMS messages are sent to individuals and are triggered in response to some action, such as a sign-up, purchase, etc.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SendTransacSmsType {
    Transactional,
    Marketing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SendTransacSmsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Transactional => serializer.serialize_str("transactional"),
            Self::Marketing => serializer.serialize_str("marketing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SendTransacSmsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transactional" => Ok(Self::Transactional),
            "marketing" => Ok(Self::Marketing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SendTransacSmsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transactional => write!(f, "transactional"),
            Self::Marketing => write!(f, "marketing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
