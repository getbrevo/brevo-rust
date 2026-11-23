pub use crate::prelude::*;

/// Authentication type for accessing the feed
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateExternalFeedRequestAuthType {
    Basic,
    Token,
    NoAuth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateExternalFeedRequestAuthType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Basic => serializer.serialize_str("basic"),
            Self::Token => serializer.serialize_str("token"),
            Self::NoAuth => serializer.serialize_str("noAuth"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateExternalFeedRequestAuthType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "basic" => Ok(Self::Basic),
            "token" => Ok(Self::Token),
            "noAuth" => Ok(Self::NoAuth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateExternalFeedRequestAuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Token => write!(f, "token"),
            Self::NoAuth => write!(f, "noAuth"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
