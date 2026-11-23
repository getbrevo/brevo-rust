pub use crate::prelude::*;

/// Filter the history based on webhook type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExportWebhooksHistoryRequestType {
    Transactional,
    Marketing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExportWebhooksHistoryRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Transactional => serializer.serialize_str("transactional"),
            Self::Marketing => serializer.serialize_str("marketing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExportWebhooksHistoryRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transactional" => Ok(Self::Transactional),
            "marketing" => Ok(Self::Marketing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExportWebhooksHistoryRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transactional => write!(f, "transactional"),
            Self::Marketing => write!(f, "marketing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
