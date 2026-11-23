pub use crate::prelude::*;

/// Status of the WhatsApp Campaign
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetWhatsAppCampaignResponseCampaignStatus {
    Draft,
    Scheduled,
    Pending,
    Approved,
    Running,
    Suspended,
    Rejected,
    Sent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetWhatsAppCampaignResponseCampaignStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Running => serializer.serialize_str("running"),
            Self::Suspended => serializer.serialize_str("suspended"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetWhatsAppCampaignResponseCampaignStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "scheduled" => Ok(Self::Scheduled),
            "pending" => Ok(Self::Pending),
            "approved" => Ok(Self::Approved),
            "running" => Ok(Self::Running),
            "suspended" => Ok(Self::Suspended),
            "rejected" => Ok(Self::Rejected),
            "sent" => Ok(Self::Sent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetWhatsAppCampaignResponseCampaignStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Pending => write!(f, "pending"),
            Self::Approved => write!(f, "approved"),
            Self::Running => write!(f, "running"),
            Self::Suspended => write!(f, "suspended"),
            Self::Rejected => write!(f, "rejected"),
            Self::Sent => write!(f, "sent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
