pub use crate::prelude::*;

/// source of the template
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateWhatsAppTemplateRequestSource {
    Automation,
    Conversations,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateWhatsAppTemplateRequestSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Automation => serializer.serialize_str("Automation"),
            Self::Conversations => serializer.serialize_str("Conversations"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateWhatsAppTemplateRequestSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Automation" => Ok(Self::Automation),
            "Conversations" => Ok(Self::Conversations),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateWhatsAppTemplateRequestSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automation => write!(f, "Automation"),
            Self::Conversations => write!(f, "Conversations"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
