pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetSmsEventsRequestEvent {
    Bounces,
    HardBounces,
    SoftBounces,
    Delivered,
    Sent,
    Accepted,
    Unsubscription,
    Replies,
    Blocked,
    Rejected,
    Skipped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetSmsEventsRequestEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bounces => serializer.serialize_str("bounces"),
            Self::HardBounces => serializer.serialize_str("hardBounces"),
            Self::SoftBounces => serializer.serialize_str("softBounces"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Unsubscription => serializer.serialize_str("unsubscription"),
            Self::Replies => serializer.serialize_str("replies"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Skipped => serializer.serialize_str("skipped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetSmsEventsRequestEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bounces" => Ok(Self::Bounces),
            "hardBounces" => Ok(Self::HardBounces),
            "softBounces" => Ok(Self::SoftBounces),
            "delivered" => Ok(Self::Delivered),
            "sent" => Ok(Self::Sent),
            "accepted" => Ok(Self::Accepted),
            "unsubscription" => Ok(Self::Unsubscription),
            "replies" => Ok(Self::Replies),
            "blocked" => Ok(Self::Blocked),
            "rejected" => Ok(Self::Rejected),
            "skipped" => Ok(Self::Skipped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetSmsEventsRequestEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounces => write!(f, "bounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Delivered => write!(f, "delivered"),
            Self::Sent => write!(f, "sent"),
            Self::Accepted => write!(f, "accepted"),
            Self::Unsubscription => write!(f, "unsubscription"),
            Self::Replies => write!(f, "replies"),
            Self::Blocked => write!(f, "blocked"),
            Self::Rejected => write!(f, "rejected"),
            Self::Skipped => write!(f, "skipped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
