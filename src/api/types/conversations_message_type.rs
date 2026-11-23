pub use crate::prelude::*;

/// `"agent"` for agents’ messages, `"visitor"` for visitors’ messages.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationsMessageType {
    Agent,
    Visitor,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationsMessageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Agent => serializer.serialize_str("agent"),
            Self::Visitor => serializer.serialize_str("visitor"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationsMessageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "agent" => Ok(Self::Agent),
            "visitor" => Ok(Self::Visitor),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationsMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent => write!(f, "agent"),
            Self::Visitor => write!(f, "visitor"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
