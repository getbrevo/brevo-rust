pub use crate::prelude::*;

/// Category of the template
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateWhatsAppTemplateRequestCategory {
    Marketing,
    Utility,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateWhatsAppTemplateRequestCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Marketing => serializer.serialize_str("MARKETING"),
            Self::Utility => serializer.serialize_str("UTILITY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateWhatsAppTemplateRequestCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MARKETING" => Ok(Self::Marketing),
            "UTILITY" => Ok(Self::Utility),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateWhatsAppTemplateRequestCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Marketing => write!(f, "MARKETING"),
            Self::Utility => write!(f, "UTILITY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
