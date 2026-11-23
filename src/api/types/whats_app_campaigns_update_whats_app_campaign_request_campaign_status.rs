pub use crate::prelude::*;

/// Status of the campaign
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateWhatsAppCampaignRequestCampaignStatus {
    Scheduled,
    Suspended,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateWhatsAppCampaignRequestCampaignStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Suspended => serializer.serialize_str("suspended"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateWhatsAppCampaignRequestCampaignStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "scheduled" => Ok(Self::Scheduled),
            "suspended" => Ok(Self::Suspended),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateWhatsAppCampaignRequestCampaignStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scheduled => write!(f, "scheduled"),
            Self::Suspended => write!(f, "suspended"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
