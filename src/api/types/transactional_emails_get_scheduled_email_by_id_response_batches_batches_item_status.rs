pub use crate::prelude::*;

/// Current status of the scheduled batch
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetScheduledEmailByIdResponseBatchesBatchesItemStatus {
    InProgress,
    Queued,
    Processed,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetScheduledEmailByIdResponseBatchesBatchesItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InProgress => serializer.serialize_str("inProgress"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::Processed => serializer.serialize_str("processed"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetScheduledEmailByIdResponseBatchesBatchesItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inProgress" => Ok(Self::InProgress),
            "queued" => Ok(Self::Queued),
            "processed" => Ok(Self::Processed),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetScheduledEmailByIdResponseBatchesBatchesItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress => write!(f, "inProgress"),
            Self::Queued => write!(f, "queued"),
            Self::Processed => write!(f, "processed"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
