pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetScheduledEmailByIdRequestStatus {
    Processed,
    InProgress,
    Queued,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetScheduledEmailByIdRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Processed => serializer.serialize_str("processed"),
            Self::InProgress => serializer.serialize_str("inProgress"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetScheduledEmailByIdRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "processed" => Ok(Self::Processed),
            "inProgress" => Ok(Self::InProgress),
            "queued" => Ok(Self::Queued),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetScheduledEmailByIdRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Processed => write!(f, "processed"),
            Self::InProgress => write!(f, "inProgress"),
            Self::Queued => write!(f, "queued"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
