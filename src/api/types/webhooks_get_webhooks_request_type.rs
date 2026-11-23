pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetWebhooksRequestType {
    Marketing,
    Transactional,
    Inbound,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetWebhooksRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Marketing => serializer.serialize_str("marketing"),
            Self::Transactional => serializer.serialize_str("transactional"),
            Self::Inbound => serializer.serialize_str("inbound"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetWebhooksRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "marketing" => Ok(Self::Marketing),
            "transactional" => Ok(Self::Transactional),
            "inbound" => Ok(Self::Inbound),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetWebhooksRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Marketing => write!(f, "marketing"),
            Self::Transactional => write!(f, "transactional"),
            Self::Inbound => write!(f, "inbound"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
