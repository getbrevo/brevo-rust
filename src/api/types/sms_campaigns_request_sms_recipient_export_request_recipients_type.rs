pub use crate::prelude::*;

/// Filter the recipients based on how they interacted with the campaign
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestSmsRecipientExportRequestRecipientsType {
    All,
    Delivered,
    Answered,
    SoftBounces,
    HardBounces,
    Unsubscribed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RequestSmsRecipientExportRequestRecipientsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Answered => serializer.serialize_str("answered"),
            Self::SoftBounces => serializer.serialize_str("softBounces"),
            Self::HardBounces => serializer.serialize_str("hardBounces"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RequestSmsRecipientExportRequestRecipientsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "delivered" => Ok(Self::Delivered),
            "answered" => Ok(Self::Answered),
            "softBounces" => Ok(Self::SoftBounces),
            "hardBounces" => Ok(Self::HardBounces),
            "unsubscribed" => Ok(Self::Unsubscribed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RequestSmsRecipientExportRequestRecipientsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Delivered => write!(f, "delivered"),
            Self::Answered => write!(f, "answered"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
