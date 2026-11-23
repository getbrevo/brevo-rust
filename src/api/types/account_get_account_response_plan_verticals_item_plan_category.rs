pub use crate::prelude::*;

/// Category of the plan
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAccountResponsePlanVerticalsItemPlanCategory {
    Marketing,
    Chat,
    Crm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAccountResponsePlanVerticalsItemPlanCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Marketing => serializer.serialize_str("Marketing"),
            Self::Chat => serializer.serialize_str("Chat"),
            Self::Crm => serializer.serialize_str("CRM"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAccountResponsePlanVerticalsItemPlanCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Marketing" => Ok(Self::Marketing),
            "Chat" => Ok(Self::Chat),
            "CRM" => Ok(Self::Crm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAccountResponsePlanVerticalsItemPlanCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Marketing => write!(f, "Marketing"),
            Self::Chat => write!(f, "Chat"),
            Self::Crm => write!(f, "CRM"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
