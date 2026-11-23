pub use crate::prelude::*;

/// Action to perform on the association. - `link`: Create association between records (default if not provided) - `unlink`: Remove association between records. Idempotent — unlinking a non-existing association is a no-op success.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpsertrecordsRequestRecordsItemAssociationsItemAction {
    Link,
    Unlink,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpsertrecordsRequestRecordsItemAssociationsItemAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Link => serializer.serialize_str("link"),
            Self::Unlink => serializer.serialize_str("unlink"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpsertrecordsRequestRecordsItemAssociationsItemAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "link" => Ok(Self::Link),
            "unlink" => Ok(Self::Unlink),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpsertrecordsRequestRecordsItemAssociationsItemAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Link => write!(f, "link"),
            Self::Unlink => write!(f, "unlink"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
