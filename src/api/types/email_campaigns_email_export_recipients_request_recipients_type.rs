pub use crate::prelude::*;

/// Type of recipients to export for a campaign
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmailExportRecipientsRequestRecipientsType {
    All,
    NonClickers,
    NonOpeners,
    Clickers,
    Openers,
    SoftBounces,
    HardBounces,
    Unsubscribed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EmailExportRecipientsRequestRecipientsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::NonClickers => serializer.serialize_str("nonClickers"),
            Self::NonOpeners => serializer.serialize_str("nonOpeners"),
            Self::Clickers => serializer.serialize_str("clickers"),
            Self::Openers => serializer.serialize_str("openers"),
            Self::SoftBounces => serializer.serialize_str("softBounces"),
            Self::HardBounces => serializer.serialize_str("hardBounces"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EmailExportRecipientsRequestRecipientsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "nonClickers" => Ok(Self::NonClickers),
            "nonOpeners" => Ok(Self::NonOpeners),
            "clickers" => Ok(Self::Clickers),
            "openers" => Ok(Self::Openers),
            "softBounces" => Ok(Self::SoftBounces),
            "hardBounces" => Ok(Self::HardBounces),
            "unsubscribed" => Ok(Self::Unsubscribed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EmailExportRecipientsRequestRecipientsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::NonClickers => write!(f, "nonClickers"),
            Self::NonOpeners => write!(f, "nonOpeners"),
            Self::Clickers => write!(f, "clickers"),
            Self::Openers => write!(f, "openers"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
