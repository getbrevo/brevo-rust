pub use crate::prelude::*;

/// The type of object the attribute belongs to, it cannot be updated after creation
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatchCrmAttributesIdRequestObjectType {
    Companies,
    Deals,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PatchCrmAttributesIdRequestObjectType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Companies => serializer.serialize_str("companies"),
            Self::Deals => serializer.serialize_str("deals"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PatchCrmAttributesIdRequestObjectType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "companies" => Ok(Self::Companies),
            "deals" => Ok(Self::Deals),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PatchCrmAttributesIdRequestObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Companies => write!(f, "companies"),
            Self::Deals => write!(f, "deals"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
